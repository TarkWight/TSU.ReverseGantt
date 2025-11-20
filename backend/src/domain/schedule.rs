use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub ls: Option<DateTime<Utc>>, // Latest Start
    pub lf: Option<DateTime<Utc>>, // Latest Finish
    pub slack: Option<i64>, // Slack time in seconds
    pub is_critical: bool,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            ls: None,
            lf: None,
            slack: None,
            is_critical: false,
        }
    }
}
