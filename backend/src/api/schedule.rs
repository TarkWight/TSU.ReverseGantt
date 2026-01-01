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

