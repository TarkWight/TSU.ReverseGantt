use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use crate::services::ScheduleService;
use crate::utils::{AppResult, parse_id};
use crate::api::tasks::TaskResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseScheduleResponse {
    pub tasks: Vec<TaskResponse>,
}

pub async fn reverse_schedule(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn ScheduleService>>,
) -> AppResult<Json<ReverseScheduleResponse>> {
    let pid = parse_id(&project_id)?;
    let tasks = service.reverse_schedule(pid).await?;
    Ok(Json(ReverseScheduleResponse {
        tasks: tasks.into_iter().map(Into::into).collect(),
    }))
}