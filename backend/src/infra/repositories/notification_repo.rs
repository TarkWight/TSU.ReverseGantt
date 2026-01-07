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
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, notification_type, title, message, task_id, project_id, is_read, created_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch notifications")?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                let notification_type = Self::parse_notification_type(&r.notification_type)?;
                Some(Notification {
                    id: r.id,
                    user_id: r.user_id,
                    notification_type,
                    title: r.title,
                    message: r.message,
                    task_id: r.task_id,
                    project_id: r.project_id,
                    is_read: r.is_read,
                    created_at: r.created_at,
                })
            })
            .collect())
    }

    async fn count_unread(&self, user_id: Id) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) AS "count!" FROM notifications
            WHERE user_id = $1 AND is_read = false
            "#,
            user_id
        )
            .fetch_one(&self.pool)
            .await
            .context("Failed to count unread")?;

        Ok(count)
    }

    async fn insert(&self, notification: &Notification) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO notifications (id, user_id, notification_type, title, message, task_id, project_id, is_read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            notification.id,
            notification.user_id,
            notification.notification_type.to_string(),
            notification.title,
            notification.message,
            notification.task_id,
            notification.project_id,
            notification.is_read,
            notification.created_at
        )
            .execute(&self.pool)
            .await
            .context("Failed to insert notification")?;

        Ok(())
    }

    async fn mark_as_read(&self, id: Id, user_id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn mark_all_as_read(&self, user_id: Id) -> anyhow::Result<()> {
        todo!()
    }
}

