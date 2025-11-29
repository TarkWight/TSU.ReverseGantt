use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

// Router functions removed - using handlers directly in mod.rs

pub async fn login(Json(_req): Json<LoginRequest>) -> Json<LoginResponse> {
    // TODO: Implement authentication
    Json(LoginResponse {
        token: "stub_token".to_string(),
        user_id: "stub_user_id".to_string(),
    })
}