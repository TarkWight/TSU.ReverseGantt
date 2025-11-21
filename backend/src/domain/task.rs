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
    // Actual execution fields
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_finish: Option<DateTime<Utc>>,
    // Progress tracking
    pub progress: i32, // 0-100
    // Buffer and hardness
    pub buffer: i64,   // Buffer in seconds
    pub hardness: Hardness,
    // Deadline
    pub deadline: Option<DateTime<Utc>>,
    // Schedule (computed by reverse scheduling)
    pub schedule: crate::domain::Schedule,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}