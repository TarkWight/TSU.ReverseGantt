use serde::{Deserialize, Serialize};

use crate::domain::{Dependency, DepType};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDependencyRequest {
    pub to_task_id: String,
    pub dep_type: DepType,
    pub min_gap: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyResponse {
    pub id: String,
    pub from_task_id: String,
    pub to_task_id: String,
    pub dep_type: DepType,
    pub min_gap: i64,
}

impl From<Dependency> for DependencyResponse {
    fn from(dep: Dependency) -> Self {
        Self {
            id: dep.id.to_string(),
            from_task_id: dep.from_task_id.to_string(),
            to_task_id: dep.to_task_id.to_string(),
            dep_type: dep.dep_type,
            min_gap: dep.min_gap,
        }
    }
}