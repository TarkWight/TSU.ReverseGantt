use axum::{routing::get, Json, Router};
use axum::extract::{Path, State};

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::tasks::TaskResponse;
use crate::infra::errors::AppResult;
use crate::utils::parse_id;
use crate::auth::permissions::ensure_project_access;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/export/projects/{id}/tasks", get(export_tasks))
        .with_state(state)
}

async fn export_tasks(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    let pid = parse_id(&project_id)?;
    ensure_project_access(&auth, pid, &state).await?;
    let tasks = state.task_service.get_by_project(&auth, pid).await?;
    Ok(Json(tasks.into_iter().map(TaskResponse::from).collect()))
}

