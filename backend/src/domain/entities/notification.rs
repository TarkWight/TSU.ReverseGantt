use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::utils::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    TaskSubmitted,    // Task sent for review -> notify leader/teacher
    TaskAccepted,     // Task accepted -> notify owner
    TaskRejected,     // Task rejected -> notify owner
    ProjectAccepted,  // Project accepted by teacher -> notify leader
    ProjectRejected,  // Project rejected by teacher -> notify leader
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::TaskSubmitted => write!(f, "task_submitted"),
            NotificationType::TaskAccepted => write!(f, "task_accepted"),
            NotificationType::TaskRejected => write!(f, "task_rejected"),
            NotificationType::ProjectAccepted => write!(f, "project_accepted"),
            NotificationType::ProjectRejected => write!(f, "project_rejected"),
        }
    }
}

impl std::str::FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "task_submitted" => Ok(NotificationType::TaskSubmitted),
            "task_accepted" => Ok(NotificationType::TaskAccepted),
            "task_rejected" => Ok(NotificationType::TaskRejected),
            "project_accepted" => Ok(NotificationType::ProjectAccepted),
            "project_rejected" => Ok(NotificationType::ProjectRejected),
            _ => Err(format!("Unknown notification type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Id,
    pub user_id: Id,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: Option<String>,
    pub task_id: Option<Id>,
    pub project_id: Option<Id>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

