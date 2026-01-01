use serde::{Deserialize, Serialize};
use crate::domain::enums::AssignRole;
use crate::utils::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub id: Id,
    pub task_id: Id,
    pub user_id: Id,
    pub role: AssignRole,
}