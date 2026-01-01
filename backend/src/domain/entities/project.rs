use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub due_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
