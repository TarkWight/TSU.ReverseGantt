use async_trait::async_trait;
use std::sync::Arc;

use crate::auth::AuthContext;
use crate::domain::{
    Task, TaskType, TaskStatus, Priority, Hardness, Schedule,
    Assignment, AssignRole, GlobalRole,
    Notification, NotificationType, ReviewDecision,
};
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::{
    TaskRepository, MembershipRepository, AssignmentRepository, UserRepository,
};
use crate::infra::email::SmtpService;
use crate::utils::{Id, generate_id, validate_task_name};

#[async_trait]
pub trait TaskService: Send + Sync {
    async fn get_by_project(&self, auth: &AuthContext, project_id: Id) -> AppResult<Vec<Task>>;
    async fn get_by_id(&self, auth: &AuthContext, id: Id) -> AppResult<Task>;
    async fn get_by_id_internal(&self, id: Id) -> AppResult<Task>;
    async fn create(
        &self,
        auth: &AuthContext,
        project_id: Id,
        name: String,
        description: Option<String>,
        task_type: TaskType,
        priority: Priority,
        estimated_duration: Option<i64>,
        parent_task_id: Option<Id>,
        hardness: Option<Hardness>,
        buffer: Option<i64>,
        owner_id: Option<Id>,
    ) -> AppResult<Task>;
    async fn update(
        &self,
        auth: &AuthContext,
        id: Id,
        name: Option<String>,
        description: Option<String>,
        task_type: Option<TaskType>,
        status: Option<TaskStatus>,
        priority: Option<Priority>,
        estimated_duration: Option<i64>,
        progress: Option<i32>,
        buffer: Option<i64>,
        hardness: Option<Hardness>,
    ) -> AppResult<Task>;
    async fn delete(&self, auth: &AuthContext, id: Id) -> AppResult<()>;
}

pub struct TaskServiceImpl {
    task_repo: Arc<dyn TaskRepository>,
    membership_repo: Arc<dyn MembershipRepository>,
    assignment_repo: Arc<dyn AssignmentRepository>,
    user_repo: Arc<dyn UserRepository>,
    notification_service: Arc<dyn crate::services::NotificationService>,
    review_service: Arc<dyn crate::services::ReviewService>,
    email_service: Arc<SmtpService>,
}

impl TaskServiceImpl {
    pub fn new(
        task_repo: Arc<dyn TaskRepository>,
        membership_repo: Arc<dyn MembershipRepository>,
        assignment_repo: Arc<dyn AssignmentRepository>,
        user_repo: Arc<dyn UserRepository>,
        notification_service: Arc<dyn crate::services::NotificationService>,
        review_service: Arc<dyn crate::services::ReviewService>,
        email_service: Arc<SmtpService>,
    ) -> Self {
        Self {
            task_repo,
            membership_repo,
            assignment_repo,
            user_repo,
            notification_service,
            review_service,
            email_service,
        }
    }

    async fn ensure_project_access(&self, auth: &AuthContext, project_id: Id) -> AppResult<()> {
        if auth.is_teacher {
            return Ok(());
        }

        let membership = self
            .membership_repo
            .find_by_project_and_user(project_id, auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        if membership.is_none() {
            return Err(AppError::Forbidden("No access to this project".into()));
        }

        Ok(())
    }

    async fn ensure_can_create_task(&self, auth: &AuthContext, project_id: Id) -> AppResult<()> {
        self.ensure_project_access(auth, project_id).await
    }

    async fn ensure_can_edit_task(&self, auth: &AuthContext, task: &Task) -> AppResult<()> {
        if auth.is_teacher {
            return Ok(());
        }

        let membership = self
            .membership_repo
            .find_by_project_and_user(task.project_id, auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        if let Some(m) = &membership {
            if m.is_leader {
                return Ok(());
            }
        }

        let assignments = self
            .assignment_repo
            .find_by_task(task.id)
            .await
            .map_err(AppError::Internal)?;

        let is_owner = assignments.iter().any(|a| a.user_id == auth.user_id && a.role == AssignRole::Owner);
        let is_assignee = assignments.iter().any(|a| a.user_id == auth.user_id);

        if is_owner || is_assignee {
            return Ok(());
        }

        Err(AppError::Forbidden("You cannot edit this task".into()))
    }

    async fn ensure_can_delete_task(&self, auth: &AuthContext, task: &Task) -> AppResult<()> {
        if auth.is_teacher {
            return Ok(());
        }

        let membership = self
            .membership_repo
            .find_by_project_and_user(task.project_id, auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        match membership {
            Some(m) if m.is_leader => Ok(()),
            _ => Err(AppError::Forbidden("Only project leader can delete tasks".into())),
        }
    }

    async fn ensure_leader_or_teacher(&self, auth: &AuthContext, project_id: Id) -> AppResult<()> {
        if auth.is_teacher {
            return Ok(());
        }

        let membership = self
            .membership_repo
            .find_by_project_and_user(project_id, auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        match membership {
            Some(m) if m.is_leader => Ok(()),
            _ => Err(AppError::Forbidden("Only leader or teacher can do this".into())),
        }
    }

    async fn notify_on_status_change(&self, task: &Task, old_status: TaskStatus) {
        if old_status == task.status {
            return;
        }

        if task.status == TaskStatus::NeedsReview {
            self.notify_leader_about_review(task).await;
        }

        self.notify_owner_about_status(task, old_status).await;
    }

    async fn notify_leader_about_review(&self, task: &Task) {
        let memberships = match self.membership_repo.find_by_project(task.project_id).await {
            Ok(m) => m,
            Err(_) => return,
        };

        let leader = match memberships.iter().find(|m| m.is_leader) {
            Some(m) => m,
            None => return,
        };

        let leader_user = match self.user_repo.find_by_id(leader.user_id).await {
            Ok(Some(u)) => u,
            _ => return,
        };

        let assignments = self.assignment_repo.find_by_task(task.id).await.unwrap_or_default();
        let owner_name = if let Some(owner) = assignments.iter().find(|a| a.role == AssignRole::Owner) {
            self.user_repo.find_by_id(owner.user_id).await
                .ok()
                .flatten()
                .map(|u| u.name)
                .unwrap_or_else(|| "Пользователь".to_string())
        } else {
            "Пользователь".to_string()
        };

        let notification = Notification {
            id: generate_id(),
            user_id: leader_user.id,
            notification_type: NotificationType::TaskSubmitted,
            title: format!("📋 {} отправил(а) задачу на проверку", owner_name),
            message: Some(format!("Задача «{}» ожидает вашей проверки", task.name)),
            task_id: Some(task.id),
            project_id: Some(task.project_id),
            is_read: false,
            created_at: chrono::Utc::now(),
        };

        if let Err(e) = self.notification_service.create(notification).await {
            tracing::error!("Failed to notify leader: {}", e);
        }
    }

    async fn notify_owner_about_status(&self, task: &Task, old_status: TaskStatus) {
        let assignments = match self.assignment_repo.find_by_task(task.id).await {
            Ok(a) => a,
            Err(_) => return,
        };

        let owner_assignment = match assignments.iter().find(|a| a.role == AssignRole::Owner) {
            Some(a) => a,
            None => return,
        };

        let owner = match self.user_repo.find_by_id(owner_assignment.user_id).await {
            Ok(Some(u)) => u,
            _ => return,
        };

        if !owner.email_notifications_enabled {
            return;
        }

        let review = self.review_service.get_by_task(task.id).await.ok().flatten();

        let is_approved = review.as_ref()
            .and_then(|r| r.decision)
            .map(|d| d == ReviewDecision::Accepted)
            .unwrap_or(false);

        let is_rejected = review.as_ref()
            .and_then(|r| r.decision)
            .map(|d| d == ReviewDecision::Rejected)
            .unwrap_or(false);

        let status_approved = old_status == TaskStatus::NeedsReview && task.status == TaskStatus::Done;
        let status_rejected = old_status == TaskStatus::NeedsReview
            && (task.status == TaskStatus::InProgress || task.status == TaskStatus::Rejected);

        let final_is_approved = is_approved || status_approved;
        let final_is_rejected = is_rejected || status_rejected;

        let review_comment = review.and_then(|r| r.comment);
        let old_status_str = old_status.to_string();
        let new_status_str = task.status.to_string();

        let email_service = self.email_service.clone();
        let task_name = task.name.clone();
        let owner_email = owner.email.clone();
        let owner_name = owner.name.clone();

        tokio::spawn(async move {
            if let Err(e) = email_service.send_task_notification(
                &owner_email,
                &owner_name,
                &task_name,
                Some(&old_status_str),
                &new_status_str,
                review_comment.as_deref(),
                final_is_approved,
                final_is_rejected,
            ).await {
                tracing::error!("Failed to send email: {}", e);
            }
        });
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn get_by_project(&self, auth: &AuthContext, project_id: Id) -> AppResult<Vec<Task>> {
        self.ensure_project_access(auth, project_id).await?;

        self.task_repo
            .find_by_project(project_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_id(&self, auth: &AuthContext, id: Id) -> AppResult<Task> {
        let task = self.get_by_id_internal(id).await?;
        self.ensure_project_access(auth, task.project_id).await?;
        Ok(task)
    }

    async fn get_by_id_internal(&self, id: Id) -> AppResult<Task> {
        self.task_repo
            .find_by_id(id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Task {} not found", id)))
    }

    async fn create(
        &self,
        auth: &AuthContext,
        project_id: Id,
        name: String,
        description: Option<String>,
        task_type: TaskType,
        priority: Priority,
        estimated_duration: Option<i64>,
        parent_task_id: Option<Id>,
        hardness: Option<Hardness>,
        buffer: Option<i64>,
        owner_id: Option<Id>,
    ) -> AppResult<Task> {
        validate_task_name(&name)?;
        self.ensure_can_create_task(auth, project_id).await?;

        let final_hardness = if hardness.is_some() {
            self.ensure_leader_or_teacher(auth, project_id).await?;
            hardness.unwrap_or(Hardness::Soft)
        } else {
            Hardness::Soft
        };

        let task = Task {
            id: generate_id(),
            project_id,
            parent_task_id,
            name,
            description,
            task_type,
            status: TaskStatus::Planned,
            priority,
            estimated_duration,
            progress: 0,
            buffer: buffer.unwrap_or(0),
            hardness: final_hardness,
            schedule: Schedule::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.task_repo.insert(&task).await.map_err(AppError::Internal)?;

        let final_owner_id = if auth.is_teacher {
            let oid = owner_id.ok_or_else(|| {
                AppError::Validation("Teacher must specify owner_id".into())
            })?;

            let owner_user = self.user_repo
                .find_by_id(oid)
                .await
                .map_err(AppError::Internal)?
                .ok_or_else(|| AppError::NotFound("Owner not found".into()))?;

            if owner_user.global_role != GlobalRole::Student {
                return Err(AppError::Validation("Owner must be a Student".into()));
            }

            self.membership_repo
                .find_by_project_and_user(project_id, oid)
                .await
                .map_err(AppError::Internal)?
                .ok_or_else(|| AppError::Validation("Owner must be project member".into()))?;

            oid
        } else {
            auth.user_id
        };

        let assignment = Assignment {
            id: generate_id(),
            task_id: task.id,
            user_id: final_owner_id,
            role: AssignRole::Owner,
        };

        self.assignment_repo.insert(&assignment).await.map_err(AppError::Internal)?;

        Ok(task)
    }

    async fn update(
        &self,
        auth: &AuthContext,
        id: Id,
        name: Option<String>,
        description: Option<String>,
        task_type: Option<TaskType>,
        status: Option<TaskStatus>,
        priority: Option<Priority>,
        estimated_duration: Option<i64>,
        progress: Option<i32>,
        buffer: Option<i64>,
        hardness: Option<Hardness>,
    ) -> AppResult<Task> {
        let existing = self.get_by_id_internal(id).await?;
        self.ensure_can_edit_task(auth, &existing).await?;

        let old_status = existing.status.clone();
        let new_status = status.clone().unwrap_or(existing.status.clone());

        if new_status == TaskStatus::Done && old_status != TaskStatus::Done {
            let has_incomplete = self.task_repo
                .has_not_done_descendants(id)
                .await
                .map_err(AppError::Internal)?;

            if has_incomplete {
                return Err(AppError::Validation(
                    "Cannot complete task with incomplete subtasks".into(),
                ));
            }
        }

        let final_hardness = if hardness.is_some() && hardness != Some(existing.hardness) {
            self.ensure_leader_or_teacher(auth, existing.project_id).await?;
            hardness.unwrap()
        } else {
            hardness.unwrap_or(existing.hardness)
        };

        let updated_name = name.unwrap_or(existing.name.clone());
        if updated_name != existing.name {
            validate_task_name(&updated_name)?;
        }

        let updated = Task {
            id: existing.id,
            project_id: existing.project_id,
            parent_task_id: existing.parent_task_id,
            name: updated_name,
            description: description.or(existing.description),
            task_type: task_type.unwrap_or(existing.task_type),
            status: new_status,
            priority: priority.unwrap_or(existing.priority),
            estimated_duration: estimated_duration.or(existing.estimated_duration),
            progress: progress.unwrap_or(existing.progress),
            buffer: buffer.unwrap_or(existing.buffer),
            hardness: final_hardness,
            schedule: existing.schedule,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
        };

        self.task_repo.update(&updated).await.map_err(AppError::Internal)?;

        self.notify_on_status_change(&updated, old_status).await;

        Ok(updated)
    }

    async fn delete(&self, auth: &AuthContext, id: Id) -> AppResult<()> {
        let task = self.get_by_id_internal(id).await?;
        self.ensure_can_delete_task(auth, &task).await?;

        self.task_repo.delete(id).await.map_err(AppError::Internal)?;

        Ok(())
    }
}
