use axum::{
    extract::{Path, State},
    Json,
};
use std::time::Duration;

use crate::api::models::{
    LoginResponse,
    UserResponse,
};
use crate::api::requests::{
    LoginRequest,
    RegisterRequest,
    UpdateEmailNotificationsRequest,
};
use crate::state::AppState;
use crate::auth::AuthContext;
use crate::auth::permissions::ensure_teacher;
use crate::infra::errors::AppResult;
use crate::infra::security::generate_token;
use crate::utils::parse_id;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user = state.user_service.authenticate(&req.email, &req.password).await?;

    let token = generate_token(
        user.id,
        &user.global_role.to_string(),
        user.name.clone(),
        Duration::from_secs(86400),
    )?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<LoginResponse>> {
    let (user, token) = state.user_service
        .register(req.email, req.name, req.password)
        .await?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

pub async fn get_user(
    _auth: AuthContext,
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    let target_user_id = parse_id(&user_id)?;
    let user = state.user_service.get_by_id(target_user_id).await?;
    Ok(Json(user.into()))
}

pub async fn get_all_users(
    _auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<UserResponse>>> {
    let users = state.user_service.get_all().await?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
}

pub async fn promote_to_teacher(
    auth: AuthContext,
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    ensure_teacher(&auth).await?;

    let target_user_id = parse_id(&user_id)?;
    let promoted_user = state.user_service.promote_to_teacher(target_user_id).await?;

    Ok(Json(promoted_user.into()))
}

pub async fn update_email_notifications(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<UpdateEmailNotificationsRequest>,
) -> AppResult<Json<UserResponse>> {
    let updated_user = state.user_service
        .update_email_notifications(auth.user_id, req.enabled)
        .await?;
    Ok(Json(updated_user.into()))
}

