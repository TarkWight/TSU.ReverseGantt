pub mod projects;
pub mod tasks;
pub mod dependencies;
pub mod schedule;
pub mod reviews;

use std::sync::Arc;
use axum::Router;
use axum::extract::{State, Path};
use axum::Json;
use crate::utils::AppResult;
use crate::services::{
    DependencyService,
    ProjectService,
    TaskService,
    ScheduleService,
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
    dependencies::{
        CreateDependencyRequest,
        DependencyResponse,
    }
};

#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<dyn ProjectService>,
    pub task_service: Arc<dyn TaskService>,
    pub dependency_service: Arc<dyn DependencyService>,
    pub schedule_service: Arc<dyn ScheduleService>,
}

pub fn create_router(
    project_service: Box<dyn ProjectService>,
    task_service: Box<dyn TaskService>,
    dependency_service: Box<dyn DependencyService>,
    schedule_service: Box<dyn ScheduleService>,
) -> Router {
    let state = AppState {
        project_service: Arc::from(project_service),
        task_service: Arc::from(task_service),
        dependency_service: Arc::from(dependency_service),
        schedule_service: Arc::from(schedule_service),
    };

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/projects", create_projects_router().with_state(state.clone()))
        .nest("/tasks", create_tasks_router().with_state(state))
}

fn create_projects_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
               axum::routing::get(get_projects_handler)
            .post(create_project_handler),
        )
        .route(
            "/{id}",
            axum::routing::get(get_project_handler)
            .patch(update_project_handler)
            .delete(delete_project_handler),
        )
        .route(
            "/{id}/tasks",
            axum::routing::get(get_tasks_by_project_handler)
            .post(create_task_for_project_handler),
        )
        .route(
            "/{id}/schedule/reverse",
            axum::routing::post(reverse_schedule_handler),
        )
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "reverse-gantt-backend"
    }))
}

async fn get_projects_handler(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    projects::get_projects(State(state.project_service)).await
}

async fn get_project_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectResponse>> {
    projects::get_project(Path(id), State(state.project_service)).await
}

async fn create_project_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    projects::create_project(State(state.project_service), Json(req)).await
}

async fn update_project_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    projects::update_project(Path(id), State(state.project_service), Json(req)).await
}

async fn delete_project_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<axum::http::StatusCode> {
    projects::delete_project(Path(id), State(state.project_service)).await
}

fn create_tasks_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}",
            axum::routing::get(get_task_handler)
                .patch(update_task_handler)
                .delete(delete_task_handler),
        )
}

async fn get_tasks_by_project_handler(
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    tasks::get_tasks(
        Path(project_id),
        State(state.task_service),
    )
        .await
}

async fn create_task_for_project_handler(
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    tasks::create_task(
        Path(project_id),
        State(state.task_service),
        Json(req),
    )
        .await
}

async fn get_task_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<TaskResponse>> {
    tasks::get_task(
        Path(id),
        State(state.task_service),
    )
        .await
}

async fn update_task_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<TaskResponse>> {
    tasks::update_task(
        Path(id),
        State(state.task_service),
        Json(req),
    )
        .await
}

async fn delete_task_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<axum::http::StatusCode> {
    tasks::delete_task(
        Path(id),
        State(state.task_service),
    )
        .await
}

async fn get_dependencies_handler(
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<DependencyResponse>>> {
    dependencies::get_dependencies(
        Path(task_id),
        State(state.dependency_service)
    )
        .await
}

async fn create_dependency_handler(
    Path(from_task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateDependencyRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    dependencies::create_dependency(
        Path(from_task_id),
        State(state.dependency_service),
        Json(req)
    )
        .await
}

async fn reverse_schedule_handler(
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<schedule::ReverseScheduleResponse>> {
    schedule::reverse_schedule(
        Path(project_id),
        State(state.schedule_service),
    ).await
}