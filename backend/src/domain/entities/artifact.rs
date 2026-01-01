use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: Id,
    pub task_id: Id,
    pub name: String,
    pub uri: String,
    pub kind: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}