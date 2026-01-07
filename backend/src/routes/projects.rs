use axum::{routing::{get, post}, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::handlers::{projects, project_stats};
use crate::api::models::{ProjectResponse, ProjectStatsResponse};
use crate::api::requests::{CreateProjectRequest, UpdateProjectRequest};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/projects", get(get_projects).post(create_project))
        .route("/projects/{id}", get(get_project).patch(update_project).delete(delete_project))
        .route("/projects/{id}/stats", get(get_project_stats))
        .route("/projects/{id}/schedule/reverse", post(reverse_schedule))
        .with_state(state)
}

async fn get_projects(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    projects::get_projects(auth, State(state)).await
}

async fn get_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectResponse>> {
    projects::get_project(auth, Path(id), State(state)).await
}

async fn create_project(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    projects::create_project(auth, State(state), Json(req)).await
}

async fn update_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    projects::update_project(auth, Path(id), State(state), Json(req)).await
}

async fn delete_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    projects::delete_project(auth, Path(id), State(state)).await
}

async fn reverse_schedule(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<crate::api::schedule::ReverseScheduleResponse>> {
    crate::api::schedule::reverse_schedule(auth, Path(project_id), State(state)).await
}

async fn get_project_stats(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectStatsResponse>> {
    project_stats::get_project_stats(auth, Path(project_id), State(state)).await
}

