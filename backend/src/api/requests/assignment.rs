use serde::Deserialize;
use crate::domain::AssignRole;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssignmentRequest {
    pub user_id: String,
    pub role: AssignRole,
}

