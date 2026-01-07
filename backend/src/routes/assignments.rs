use axum::{routing::{get, delete}, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::assignments::{
    AssignmentResponse, CreateAssignmentRequest,
    get_task_assignments, get_user_assignments, create_assignment, delete_assignment,
};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/assignments/tasks/{task_id}", get(get_task_assignments_handler).post(create_assignment_handler))
        .route("/assignments/me", get(get_user_assignments_handler))
        .route("/assignments/{assignment_id}", delete(delete_assignment_handler))
        .with_state(state)
}

async fn get_task_assignments_handler(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<AssignmentResponse>>> {
    get_task_assignments(auth, Path(task_id), State(state)).await
}

async fn create_assignment_handler(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateAssignmentRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    create_assignment(auth, Path(task_id), State(state), Json(req)).await
}

async fn get_user_assignments_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<AssignmentResponse>>> {
    get_user_assignments(auth, State(state)).await
}

async fn delete_assignment_handler(
    auth: AuthContext,
    Path(assignment_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    delete_assignment(auth, Path(assignment_id), State(state)).await
}

