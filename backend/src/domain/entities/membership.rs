use serde::{Deserialize, Serialize};
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub id: Id,
    pub project_id: Id,
    pub user_id: Id,
    pub is_leader: bool,
    pub tags: Vec<String>,
}