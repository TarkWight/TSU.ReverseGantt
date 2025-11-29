use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::enums::ReviewDecision;
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub id: Id,
    pub task_id: Id,
    pub reviewer_id: Id,
    pub decision: Option<ReviewDecision>,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}