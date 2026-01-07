use axum::{routing::{get, delete}, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::tasks::{
    CreateTaskRequest, UpdateTaskRequest, TaskResponse,
    get_tasks, get_task, create_task, update_task, delete_task,
};
use crate::api::dependencies::{
    CreateDependencyRequest, DependencyResponse,
    get_dependencies, create_dependency, delete_dependency,
};
use crate::api::reviews::{
    CreateReviewRequest, ReviewResponse,
    get_review, create_review,
};
use crate::api::artifacts::{list_artifacts, create_artifact, delete_artifact};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        // Tasks by project
        .route("/projects/{project_id}/tasks", get(get_tasks_handler).post(create_task_handler))
        // Single task
        .route("/tasks/{id}", get(get_task_handler).patch(update_task_handler).delete(delete_task_handler))
        // Dependencies
        .route("/tasks/{id}/dependencies", get(get_dependencies_handler).post(create_dependency_handler))
        .route("/tasks/{id}/dependencies/{dep_id}", delete(delete_dependency_handler))
        // Reviews
        .route("/tasks/{id}/review", get(get_review_handler).post(create_review_handler))
        // Artifacts
        .route("/tasks/{id}/artifacts", get(list_artifacts).post(create_artifact))
        .route("/tasks/{id}/artifacts/{artifact_id}", delete(delete_artifact))
        .with_state(state)
}

async fn get_tasks_handler(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    get_tasks(auth, Path(project_id), State(state)).await
}

async fn create_task_handler(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    create_task(auth, Path(project_id), State(state), Json(req)).await
}

async fn get_task_handler(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<TaskResponse>> {
    get_task(auth, Path(id), State(state)).await
}

async fn update_task_handler(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<TaskResponse>> {
    update_task(auth, Path(id), State(state), Json(req)).await
}

async fn delete_task_handler(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    delete_task(auth, Path(id), State(state)).await
}

async fn get_dependencies_handler(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<DependencyResponse>>> {
    get_dependencies(auth, Path(task_id), State(state)).await
}

async fn create_dependency_handler(
    auth: AuthContext,
    Path(from_task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateDependencyRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    create_dependency(auth, Path(from_task_id), State(state), Json(req)).await
}

async fn delete_dependency_handler(
    auth: AuthContext,
    Path((task_id, dep_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    delete_dependency(auth, Path((task_id, dep_id)), State(state)).await
}

async fn get_review_handler(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Option<ReviewResponse>>> {
    get_review(auth, Path(task_id), State(state)).await
}

async fn create_review_handler(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateReviewRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    create_review(auth, Path(task_id), State(state), Json(req)).await
}

