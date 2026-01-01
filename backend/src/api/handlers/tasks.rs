use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::api::models::TaskResponse;
use crate::api::requests::{
    CreateTaskRequest,
    UpdateTaskRequest,
};
use crate::state::AppState;
use crate::auth::AuthContext;
use crate::infra::errors::AppResult;
use crate::utils::parse_id;

pub async fn get_tasks(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    let project_id = parse_id(&project_id)?;
    let tasks = state.task_service.get_by_project(&auth, project_id).await?;
    Ok(Json(tasks.into_iter().map(Into::into).collect()))
}

pub async fn get_task(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<TaskResponse>> {
    let task_id = parse_id(&id)?;
    let task = state.task_service.get_by_id(&auth, task_id).await?;
    Ok(Json(task.into()))
}

pub async fn create_task(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<impl IntoResponse> {
    let project_id = parse_id(&project_id)?;
    let parent_task_id = req.parent_task_id
        .as_ref().and_then(|s| parse_id(s).ok());
    let owner_id = req.owner_id
        .as_ref().and_then(|s| parse_id(s).ok());

    let task = state.task_service.create(
        &auth,
        project_id,
        req.name,
        req.description,
        req.task_type,
        req.priority,
        req.estimated_duration,
        parent_task_id,
        req.hardness,
        req.buffer,
        owner_id,
    ).await?;

    Ok((StatusCode::CREATED, Json(TaskResponse::from(task))))
}

pub async fn update_task(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<TaskResponse>> {
    let task_id = parse_id(&id)?;

    let task = state.task_service.update(
        &auth,
        task_id,
        req.name,
        req.description,
        req.task_type,
        req.status,
        req.priority,
        req.estimated_duration,
        req.progress,
        req.buffer,
        req.hardness,
    ).await?;

    Ok(Json(task.into()))
}

pub async fn delete_task(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let task_id = parse_id(&id)?;
    state.task_service.delete(&auth, task_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

