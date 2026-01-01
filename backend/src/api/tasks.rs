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