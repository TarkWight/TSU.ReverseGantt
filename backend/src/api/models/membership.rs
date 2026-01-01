use serde::Serialize;
use crate::domain::Membership;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipResponse {
    pub id: String,
    pub project_id: String,
    pub user_id: String,
    pub is_leader: bool,
    pub tags: Vec<String>,
}

impl From<Membership> for MembershipResponse {
    fn from(membership: Membership) -> Self {
        Self {
            id: membership.id.to_string(),
            project_id: membership.project_id.to_string(),
            user_id: membership.user_id.to_string(),
            is_leader: membership.is_leader,
            tags: membership.tags,
        }
    }
}

