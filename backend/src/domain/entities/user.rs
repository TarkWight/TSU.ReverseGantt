use serde::{Deserialize, Serialize};
use crate::utils::Id;
use crate::domain::enums::GlobalRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Id,
    pub email: String,
    pub name: String,
    pub global_role: GlobalRole,
    pub email_notifications_enabled: bool,
}

