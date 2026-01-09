use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::enums::{Priority, TaskStatus, TaskType, Hardness};
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Id,
    pub project_id: Id,
    pub parent_task_id: Option<Id>,
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: Priority,
    pub estimated_duration: Option<i64>, // Duration in seconds
    pub progress: i32, // 0-100
    pub buffer: i64,   // Buffer in seconds
    pub hardness: Hardness,
    pub schedule: crate::domain::Schedule,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}