use axum::{
    extract::{Path, State},
    Json,
};

use crate::infra::errors::AppResult;
use crate::utils::parse_id;
use crate::auth::AuthContext;
use crate::auth::permissions::ensure_project_access;
use crate::state::AppState;
use crate::api::models::ReverseScheduleResponse;

pub async fn reverse_schedule(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ReverseScheduleResponse>> {
    let pid = parse_id(&project_id)?;
    ensure_project_access(&auth, pid, &state).await?;

    let tasks = state.schedule_service.reverse_schedule(pid).await?;
    Ok(Json(ReverseScheduleResponse {
        tasks: tasks.into_iter().map(Into::into).collect(),
    }))
}

