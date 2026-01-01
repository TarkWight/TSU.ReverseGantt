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




pub async fn create_task(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<impl IntoResponse> {
    crate::utils::validate_task_name(&req.name)?;

    let task = Task {
        id: crate::utils::generate_id(),
        project_id: parse_id(&project_id)?,
        parent_task_id: req.parent_task_id.and_then(|s| parse_id(&s).ok()),
        name: req.name,
        description: req.description,
        task_type: req.task_type,
        status: TaskStatus::Planned,
        priority: req.priority,
        estimated_duration: req.estimated_duration,
        planned_start: None,
        planned_finish: None,
        actual_start: None,
        actual_finish: None,
        progress: 0,
        buffer: req.buffer.unwrap_or(0),
        hardness: req
            .hardness
            .unwrap_or(crate::domain::enums::Hardness::Soft),
        deadline: req.deadline,
        schedule: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = service.create(task).await?;
    Ok((StatusCode::CREATED, Json(TaskResponse::from(created))))
}

pub async fn update_task(
    Path(id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<TaskResponse>> {
    let task_id = parse_id(&id)?;
    let existing = service.get_by_id(task_id).await?;

    let updated = Task {
        id: existing.id,
        project_id: existing.project_id,
        parent_task_id: existing.parent_task_id,
        name: req.name.unwrap_or(existing.name),
        description: req.description.or(existing.description),
        task_type: req.task_type.unwrap_or(existing.task_type),
        status: req.status.unwrap_or(existing.status),
        priority: req.priority.unwrap_or(existing.priority),
        estimated_duration: req.estimated_duration.or(existing.estimated_duration),
        planned_start: req.planned_start.or(existing.planned_start),
        planned_finish: req.planned_finish.or(existing.planned_finish),
        actual_start: req.actual_start.or(existing.actual_start),
        actual_finish: req.actual_finish.or(existing.actual_finish),
        progress: req.progress.unwrap_or(existing.progress),
        buffer: req.buffer.unwrap_or(existing.buffer),
        hardness: req.hardness.unwrap_or(existing.hardness),
        deadline: req.deadline.or(existing.deadline),
        schedule: existing.schedule,
        created_at: existing.created_at,
        updated_at: chrono::Utc::now(),
    };

    let task = service.update(task_id, updated).await?;
    Ok(Json(task.into()))
}

pub async fn delete_task(
    Path(id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
) -> AppResult<StatusCode> {
    let task_id = parse_id(&id)?;
    service.delete(task_id).await?;
    Ok(StatusCode::NO_CONTENT)
}