use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::Assignment;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::AssignmentRepository;
use crate::utils::Id;

#[async_trait]
pub trait AssignmentService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Assignment>>;
    async fn get_by_user(&self, user_id: Id) -> AppResult<Vec<Assignment>>;
    async fn get_by_id(&self, assignment_id: Id) -> AppResult<Assignment>;
    async fn get_by_task_and_user(&self, task_id: Id, user_id: Id) -> AppResult<Option<Assignment>>;
    async fn create(&self, assignment: Assignment) -> AppResult<Assignment>;
    async fn delete(&self, assignment_id: Id) -> AppResult<()>;
    async fn delete_by_task_and_user(&self, task_id: Id, user_id: Id) -> AppResult<()>;
}

pub struct AssignmentServiceImpl {
    assignment_repo: Arc<dyn AssignmentRepository>,
}

impl AssignmentServiceImpl {
    pub fn new(assignment_repo: Arc<dyn AssignmentRepository>) -> Self {
        Self { assignment_repo }
    }
}

#[async_trait]
impl AssignmentService for AssignmentServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Assignment>> {
        self.assignment_repo
            .find_by_task(task_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_user(&self, user_id: Id) -> AppResult<Vec<Assignment>> {
        self.assignment_repo
            .find_by_user(user_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_id(&self, assignment_id: Id) -> AppResult<Assignment> {
        self.assignment_repo
            .find_by_id(assignment_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Assignment {} not found", assignment_id)))
    }

    async fn get_by_task_and_user(&self, task_id: Id, user_id: Id) -> AppResult<Option<Assignment>> {
        self.assignment_repo
            .find_by_task_and_user(task_id, user_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn create(&self, assignment: Assignment) -> AppResult<Assignment> {
        self.assignment_repo
            .insert(&assignment)
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("duplicate") || err_str.contains("unique") {
                    AppError::Validation(format!(
                        "Assignment already exists for task {} and user {}",
                        assignment.task_id, assignment.user_id
                    ))
                } else {
                    AppError::Internal(e)
                }
            })?;

        Ok(assignment)
    }

    async fn delete(&self, assignment_id: Id) -> AppResult<()> {
        let deleted = self.assignment_repo
            .delete(assignment_id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!("Assignment {} not found", assignment_id)));
        }

        Ok(())
    }

    async fn delete_by_task_and_user(&self, task_id: Id, user_id: Id) -> AppResult<()> {
        let deleted = self.assignment_repo
            .delete_by_task_and_user(task_id, user_id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!(
                "Assignment not found for task {} and user {}",
                task_id, user_id
            )));
        }

        Ok(())
    }
}
