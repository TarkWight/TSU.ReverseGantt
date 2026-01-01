pub mod projects;
pub mod tasks;
pub mod dependencies;
pub mod schedule;
pub mod reviews;
pub mod users;
pub mod models;

use std::sync::Arc;

use axum::{extract::{Path, State}, Json, Router};
use tower_http::cors::CorsLayer;

use crate::services::{
    DependencyService,
    ProjectService,
    TaskService,
    ScheduleService,
    ReviewService,
    UserService,
};
use crate::utils::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<dyn ProjectService>,
    pub task_service: Arc<dyn TaskService>,
    pub dependency_service: Arc<dyn DependencyService>,
    pub schedule_service: Arc<dyn ScheduleService>,
    pub review_service: Arc<dyn ReviewService>,
    pub user_service: Arc<dyn UserService>,
}

pub fn create_router(
    project_service: Box<dyn ProjectService>,
    task_service: Box<dyn TaskService>,
    dependency_service: Box<dyn DependencyService>,
    schedule_service: Box<dyn ScheduleService>,
    review_service: Box<dyn ReviewService>,
    user_service: Box<dyn UserService>,
) -> Router {
    let state = AppState {
        project_service: Arc::from(project_service),
        task_service: Arc::from(task_service),
        dependency_service: Arc::from(dependency_service),
        schedule_service: Arc::from(schedule_service),
        review_service: Arc::from(review_service),
        user_service: Arc::from(user_service),
    };

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/export", create_export_router())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

fn create_export_router() -> Router<AppState> {
    Router::new()
        .route("/projects/{id}/tasks", axum::routing::get(export_tasks))
}
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "reverse-gantt-backend"
    }))
}

async fn export_tasks(
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    let pid = crate::utils::parse_id(&project_id)?;
    let tasks = state.task_service.get_by_project(pid).await?;
    Ok(Json(tasks.into_iter().map(TaskResponse::from).collect()))
}