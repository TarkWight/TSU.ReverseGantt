use serde::Serialize;
use crate::domain::{Notification, NotificationType};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: Option<String>,
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub is_read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id.to_string(),
            notification_type: n.notification_type,
            title: n.title,
            message: n.message,
            task_id: n.task_id.map(|id| id.to_string()),
            project_id: n.project_id.map(|id| id.to_string()),
            is_read: n.is_read,
            created_at: n.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub unread_count: i64,
}

