use serde::Deserialize;
use crate::domain::DepType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDependencyRequest {
    pub to_task_id: String,
    pub dep_type: DepType,
    pub min_gap: i64,
}