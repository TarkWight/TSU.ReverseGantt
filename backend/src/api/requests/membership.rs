use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMembershipRequest {
    pub user_id: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMembershipTagsRequest {
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeProjectLeaderRequest {
    pub new_leader_id: String,
}

