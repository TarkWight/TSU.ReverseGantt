use serde::Serialize;
use crate::domain::{Assignment, AssignRole};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentResponse {
    pub id: String,
    pub task_id: String,
    pub user_id: String,
    pub role: AssignRole,
}

impl From<Assignment> for AssignmentResponse {
    fn from(assignment: Assignment) -> Self {
        Self {
            id: assignment.id.to_string(),
            task_id: assignment.task_id.to_string(),
            user_id: assignment.user_id.to_string(),
            role: assignment.role,
        }
    }
}

