use serde::Serialize;
use crate::domain::PasswordResetCode;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetRequestResponse {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
    pub request_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<(PasswordResetCode, String, String)> for PasswordResetRequestResponse {
    fn from((reset, user_name, user_email): (PasswordResetCode, String, String)) -> Self {
        Self {
            id: reset.id.to_string(),
            user_id: reset.user_id.to_string(),
            user_name,
            user_email,
            request_type: reset.request_type.to_string(),
            status: reset.status.to_string(),
            created_at: reset.created_at,
        }
    }
}


