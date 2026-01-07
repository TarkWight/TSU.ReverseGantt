use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::Notification;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::NotificationRepository;
use crate::utils::Id;

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn get_by_user(&self, user_id: Id, limit: i64) -> AppResult<Vec<Notification>>;
    async fn get_unread_count(&self, user_id: Id) -> AppResult<i64>;
    async fn create(&self, notification: Notification) -> AppResult<Notification>;
    async fn mark_as_read(&self, id: Id, user_id: Id) -> AppResult<()>;
    async fn mark_all_as_read(&self, user_id: Id) -> AppResult<()>;
}

pub struct NotificationServiceImpl {
    notification_repo: Arc<dyn NotificationRepository>,
}

impl NotificationServiceImpl {
    pub fn new(notification_repo: Arc<dyn NotificationRepository>) -> Self {
        Self { notification_repo }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn get_by_user(&self, user_id: Id, limit: i64) -> AppResult<Vec<Notification>> {
        self.notification_repo
            .find_by_user(user_id, limit)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_unread_count(&self, user_id: Id) -> AppResult<i64> {
        self.notification_repo
            .count_unread(user_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn create(&self, notification: Notification) -> AppResult<Notification> {
        self.notification_repo
            .insert(&notification)
            .await
            .map_err(AppError::Internal)?;

        Ok(notification)
    }

    async fn mark_as_read(&self, id: Id, user_id: Id) -> AppResult<()> {
        let updated = self.notification_repo
            .mark_as_read(id, user_id)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("Notification {} not found", id)));
        }

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Id) -> AppResult<()> {
        self.notification_repo
            .mark_all_as_read(user_id)
            .await
            .map_err(AppError::Internal)
    }
}
