use serde::{Deserialize, Serialize};
use crate::domain::enums::ProjectRole;
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub id: Id,
    pub project_id: Id,
    pub user_id: Id,
    pub role: ProjectRole,
    pub is_leader: bool,
}