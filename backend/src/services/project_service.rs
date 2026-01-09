use async_trait::async_trait;
use std::sync::Arc;

use crate::auth::AuthContext;
use crate::domain::{Project, Membership, GlobalRole};
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::{ProjectRepository, MembershipRepository, UserRepository};
use crate::utils::{Id, generate_id, validate_project_name};

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn get_all_for_user(&self, auth: &AuthContext) -> AppResult<Vec<Project>>;
    async fn get_by_id(&self, auth: &AuthContext, id: Id) -> AppResult<Project>;
    async fn create(&self, auth: &AuthContext, project: Project, leader_id: Id) -> AppResult<Project>;
    async fn update(
        &self,
        auth: &AuthContext,
        id: Id,
        name: Option<String>,
        description: Option<String>,
        start_date: Option<chrono::NaiveDate>,
        due_date: Option<chrono::NaiveDate>,
    ) -> AppResult<Project>;
    async fn delete(&self, auth: &AuthContext, id: Id) -> AppResult<()>;
}

pub struct ProjectServiceImpl {
    project_repo: Arc<dyn ProjectRepository>,
    membership_repo: Arc<dyn MembershipRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ProjectServiceImpl {
    pub fn new(
        project_repo: Arc<dyn ProjectRepository>,
        membership_repo: Arc<dyn MembershipRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            project_repo,
            membership_repo,
            user_repo,
        }
    }

    async fn ensure_access(&self, auth: &AuthContext, project_id: Id) -> AppResult<()> {
        if auth.is_teacher {
            return Ok(());
        }

        let membership = self
            .membership_repo
            .find_by_project_and_user(project_id, auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        if membership.is_none() {
            return Err(AppError::Forbidden("You do not have access to this project".into()));
        }

        Ok(())
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
            _ => Err(AppError::Forbidden("You must be the project leader".into())),
        }
    }
}

#[async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn get_all_for_user(&self, auth: &AuthContext) -> AppResult<Vec<Project>> {
        let all_projects = self
            .project_repo
            .find_all()
            .await
            .map_err(AppError::Internal)?;

        if auth.is_teacher {
            return Ok(all_projects);
        }

        let user_memberships = self
            .membership_repo
            .find_by_user(auth.user_id)
            .await
            .map_err(AppError::Internal)?;

        let user_project_ids: std::collections::HashSet<Id> =
            user_memberships.iter().map(|m| m.project_id).collect();

        Ok(all_projects
            .into_iter()
            .filter(|p| user_project_ids.contains(&p.id))
            .collect())
    }

    async fn get_by_id(&self, auth: &AuthContext, id: Id) -> AppResult<Project> {
        self.ensure_access(auth, id).await?;

        self.project_repo
            .find_by_id(id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))
    }

    async fn create(&self, auth: &AuthContext, project: Project, leader_id: Id) -> AppResult<Project> {
        validate_project_name(&project.name)?;

        if auth.is_teacher {
            let leader = self
                .user_repo
                .find_by_id(leader_id)
                .await
                .map_err(AppError::Internal)?
                .ok_or_else(|| AppError::NotFound("Leader user not found".into()))?;

            if leader.global_role != GlobalRole::Student {
                return Err(AppError::Validation("Leader must be a Student".into()));
            }

            if leader_id == auth.user_id {
                return Err(AppError::Validation("Teacher cannot be project leader".into()));
            }
        }

        self.project_repo
            .insert(&project)
            .await
            .map_err(AppError::Internal)?;

        let membership = Membership {
            id: generate_id(),
            project_id: project.id,
            user_id: leader_id,
            is_leader: true,
            tags: vec![],
        };

        self.membership_repo
            .insert(&membership)
            .await
            .map_err(AppError::Internal)?;

        Ok(project)
    }

    async fn update(
        &self,
        auth: &AuthContext,
        id: Id,
        name: Option<String>,
        description: Option<String>,
        start_date: Option<chrono::NaiveDate>,
        due_date: Option<chrono::NaiveDate>,
    ) -> AppResult<Project> {
        self.ensure_leader_or_teacher(auth, id).await?;

        let existing = self
            .project_repo
            .find_by_id(id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;

        let updated_name = name.unwrap_or(existing.name);
        validate_project_name(&updated_name)?;

        let updated = Project {
            id: existing.id,
            name: updated_name,
            description: description.or(existing.description),
            start_date: start_date.or(existing.start_date),
            due_date: due_date.unwrap_or(existing.due_date),
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
        };

        self.project_repo
            .update(&updated)
            .await
            .map_err(AppError::Internal)?;

        Ok(updated)
    }

    async fn delete(&self, auth: &AuthContext, id: Id) -> AppResult<()> {
        self.ensure_leader_or_teacher(auth, id).await?;

        let deleted = self
            .project_repo
            .delete(id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!("Project {} not found", id)));
        }

        Ok(())
    }
}
