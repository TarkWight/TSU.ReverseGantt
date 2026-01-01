use serde::Deserialize;
use crate::domain::{
    TaskType,
    TaskStatus,
    Priority,
    Hardness,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub priority: Priority,
    pub estimated_duration: Option<i64>,
    pub parent_task_id: Option<String>,
    pub hardness: Option<Hardness>,
    pub buffer: Option<i64>,
    #[serde(default)]
    pub owner_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub task_type: Option<TaskType>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub estimated_duration: Option<i64>,
    pub progress: Option<i32>,
    pub buffer: Option<i64>,
    pub hardness: Option<Hardness>,
}