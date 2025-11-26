pub mod projects;
pub mod tasks;

use std::sync::Arc;
use axum::Router;
use axum::extract::State;
use axum::Json;
use crate::utils::AppResult;
use crate::services::{
    ProjectService,
    TaskService,
};

use crate::api::{
    projects::{
        CreateProjectRequest,
        UpdateProjectRequest,
        ProjectResponse,
    },
    tasks::{
        CreateTaskRequest,
        UpdateTaskRequest,
        TaskResponse,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<dyn ProjectService>,
    pub task_service: Arc<dyn TaskService>,
}

pub fn create_router(
    project_service: Box<dyn ProjectService>,
    task_service: Box<dyn TaskService>,
) -> Router {
    let state = AppState {
        project_service: Arc::from(project_service),
        task_service: Arc::from(task_service),
    };

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/projects", create_projects_router().with_state(state))
}

fn create_projects_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            axum::routing::get(get_projects_handler).post(create_project_handler),
        )
        .route(
            "/{id}",
            axum::routing::get(get_project_handler)
                .patch(update_project_handler)
                .delete(delete_project_handler),
        )
}

// simple health endpoint for the backend
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "reverse-gantt-backend"
    }))
}

// wrappers that bridge AppState to per-module handlers

async fn get_projects_handler(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    projects::get_projects(State(state.project_service)).await
}

async fn get_project_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectResponse>> {
    projects::get_project(axum::extract::Path(id), State(state.project_service)).await
}

async fn create_project_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    projects::create_project(State(state.project_service), Json(req)).await
}

async fn update_project_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    projects::update_project(axum::extract::Path(id), State(state.project_service), Json(req)).await
}

async fn delete_project_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> AppResult<axum::http::StatusCode> {
    projects::delete_project(axum::extract::Path(id), State(state.project_service)).await
}
