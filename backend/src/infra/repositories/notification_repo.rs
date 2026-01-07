use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{Notification, NotificationType};
use crate::utils::Id;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn find_by_user(&self, user_id: Id, limit: i64) -> anyhow::Result<Vec<Notification>>;
    async fn count_unread(&self, user_id: Id) -> anyhow::Result<i64>;
    async fn insert(&self, notification: &Notification) -> anyhow::Result<()>;
    async fn mark_as_read(&self, id: Id, user_id: Id) -> anyhow::Result<bool>;
    async fn mark_all_as_read(&self, user_id: Id) -> anyhow::Result<()>;
}

pub struct PgNotificationRepository {
    pool: PgPool,
}

impl PgNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_notification_type(s: &str) -> Option<NotificationType> {
        s.parse().ok()
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn find_by_user(&self, user_id: Id, limit: i64) -> anyhow::Result<Vec<Notification>> {
        todo!()
    }

    async fn count_unread(&self, user_id: Id) -> anyhow::Result<i64> {
        todo!()
    }

    async fn insert(&self, notification: &Notification) -> anyhow::Result<()> {
        todo!()
    }

    async fn mark_as_read(&self, id: Id, user_id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn mark_all_as_read(&self, user_id: Id) -> anyhow::Result<()> {
        todo!()
    }
}

