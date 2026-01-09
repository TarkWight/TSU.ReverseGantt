use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub id: Id,
    pub task_id: Id,
    pub score: i32, // Numeric risk value
    pub reason: String,
    pub mitigation: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}