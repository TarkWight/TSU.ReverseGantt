use serde::Serialize;
use crate::domain::{Task, TaskType, TaskStatus, Priority, Hardness, Schedule};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub id: String,
    pub project_id: String,
    pub parent_task_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: Priority,
    pub estimated_duration: Option<i64>,
    pub progress: i32,
    pub buffer: i64,
    pub hardness: Hardness,
    pub schedule: Schedule,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Task> for TaskResponse {
    fn from(t: Task) -> Self {
        Self {
            id: t.id.to_string(),
            project_id: t.project_id.to_string(),
            parent_task_id: t.parent_task_id.map(|id| id.to_string()),
            name: t.name,
            description: t.description,
            task_type: t.task_type,
            status: t.status,
            priority: t.priority,
            estimated_duration: t.estimated_duration,
            progress: t.progress,
            buffer: t.buffer,
            hardness: t.hardness,
            schedule: t.schedule,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}