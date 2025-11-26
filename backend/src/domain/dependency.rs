use serde::{Deserialize, Serialize};
use crate::domain::enums::DepType;
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: Id,
    pub from_task_id: Id,
    pub to_task_id: Id,
    pub dep_type: DepType,
    pub min_gap: i64, // Minimum gap in seconds
}