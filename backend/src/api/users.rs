use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use crate::services::UserService;
use crate::utils::{AppResult, Id, jwt::generate_token};

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}



pub async fn register(
    State(service): State<Arc<dyn UserService>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<LoginResponse>> {
    let (user, token) = service
        .register(req.email, req.name, req.password)
        .await?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id.to_string(),
    }))
}