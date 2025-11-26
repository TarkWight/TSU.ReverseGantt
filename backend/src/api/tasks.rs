use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::domain::{Task, TaskType, TaskStatus, Priority};
use crate::services::TaskService;
use crate::utils::{AppResult, parse_id};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub priority: Priority,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub estimated_duration: Option<i64>,
    pub parent_task_id: Option<String>,
    pub hardness: Option<crate::domain::enums::Hardness>,
    pub buffer: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub task_type: Option<TaskType>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub estimated_duration: Option<i64>,
    pub planned_start: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_finish: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_start: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_finish: Option<chrono::DateTime<chrono::Utc>>,
    pub progress: Option<i32>,
    pub buffer: Option<i64>,
    pub hardness: Option<crate::domain::enums::Hardness>,
}

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
    pub planned_start: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_finish: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_start: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_finish: Option<chrono::DateTime<chrono::Utc>>,
    pub progress: i32,
    pub buffer: i64,
    pub hardness: crate::domain::enums::Hardness,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub schedule: crate::domain::Schedule,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.to_string(),
            project_id: task.project_id.to_string(),
            parent_task_id: task.parent_task_id.map(|id| id.to_string()),
            name: task.name,
            description: task.description,
            task_type: task.task_type,
            status: task.status,
            priority: task.priority,
            estimated_duration: task.estimated_duration,
            planned_start: task.planned_start,
            planned_finish: task.planned_finish,
            actual_start: task.actual_start,
            actual_finish: task.actual_finish,
            progress: task.progress,
            buffer: task.buffer,
            hardness: task.hardness,
            deadline: task.deadline,
            schedule: task.schedule,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}