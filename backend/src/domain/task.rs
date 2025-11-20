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
    // Planning fields
    pub estimated_duration: Option<i64>, // Duration in seconds
    pub planned_start: Option<DateTime<Utc>>,
    pub planned_finish: Option<DateTime<Utc>>,
}