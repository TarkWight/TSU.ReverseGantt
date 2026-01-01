use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::domain::{Dependency, DepType};
use crate::services::DependencyService;
use crate::utils::{AppResult, parse_id};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDependencyRequest {
    pub to_task_id: String,
    pub dep_type: DepType,
    pub min_gap: i64,
}

