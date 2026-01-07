use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::Membership;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::MembershipRepository;
use crate::utils::Id;

#[async_trait]
pub trait MembershipService: Send + Sync {
    async fn get_by_project(&self, project_id: Id) -> AppResult<Vec<Membership>>;
    async fn get_by_user(&self, user_id: Id) -> AppResult<Vec<Membership>>;
    async fn get_by_id(&self, membership_id: Id) -> AppResult<Membership>;
    async fn get_by_project_and_user(&self, project_id: Id, user_id: Id) -> AppResult<Membership>;
    async fn create(&self, membership: Membership) -> AppResult<Membership>;
    async fn update_tags(&self, membership_id: Id, tags: Vec<String>) -> AppResult<Membership>;
    async fn update_leader_status(&self, membership_id: Id, is_leader: bool) -> AppResult<Membership>;
    async fn delete(&self, membership_id: Id) -> AppResult<()>;
    async fn check_user_is_leader(&self, project_id: Id, user_id: Id) -> AppResult<bool>;
    async fn get_leader(&self, project_id: Id) -> AppResult<Option<Membership>>;
}

pub struct MembershipServiceImpl {
    membership_repo: Arc<dyn MembershipRepository>,
}

impl MembershipServiceImpl {
    pub fn new(membership_repo: Arc<dyn MembershipRepository>) -> Self {
        Self { membership_repo }
    }
}

#[async_trait]
impl MembershipService for MembershipServiceImpl {
    async fn get_by_project(&self, project_id: Id) -> AppResult<Vec<Membership>> {
        self.membership_repo
            .find_by_project(project_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_user(&self, user_id: Id) -> AppResult<Vec<Membership>> {
        self.membership_repo
            .find_by_user(user_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_id(&self, membership_id: Id) -> AppResult<Membership> {
        self.membership_repo
            .find_by_id(membership_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Membership {} not found", membership_id)))
    }

    async fn get_by_project_and_user(&self, project_id: Id, user_id: Id) -> AppResult<Membership> {
        self.membership_repo
            .find_by_project_and_user(project_id, user_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!(
                "Membership not found for project {} and user {}", project_id, user_id
            )))
    }

    async fn create(&self, membership: Membership) -> AppResult<Membership> {
        self.membership_repo
            .insert(&membership)
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("unique_project_user") {
                    AppError::Validation(format!(
                        "User {} is already a member of project {}",
                        membership.user_id, membership.project_id
                    ))
                } else {
                    AppError::Internal(e)
                }
            })?;

        Ok(membership)
    }

    async fn update_tags(&self, membership_id: Id, tags: Vec<String>) -> AppResult<Membership> {
        let updated = self.membership_repo
            .update_tags(membership_id, &tags)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("Membership {} not found", membership_id)));
        }

        self.get_by_id(membership_id).await
    }

    async fn update_leader_status(&self, membership_id: Id, is_leader: bool) -> AppResult<Membership> {
        let updated = self.membership_repo
            .update_leader(membership_id, is_leader)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("Membership {} not found", membership_id)));
        }

        self.get_by_id(membership_id).await
    }

    async fn delete(&self, membership_id: Id) -> AppResult<()> {
        let deleted = self.membership_repo
            .delete(membership_id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!("Membership {} not found", membership_id)));
        }

        Ok(())
    }

    async fn check_user_is_leader(&self, project_id: Id, user_id: Id) -> AppResult<bool> {
        let membership = self.membership_repo
            .find_by_project_and_user(project_id, user_id)
            .await
            .map_err(AppError::Internal)?;

        Ok(membership.map(|m| m.is_leader).unwrap_or(false))
    }

    async fn get_leader(&self, project_id: Id) -> AppResult<Option<Membership>> {
        self.membership_repo
            .find_leader(project_id)
            .await
            .map_err(AppError::Internal)
    }
}
