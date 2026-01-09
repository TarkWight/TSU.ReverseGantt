use axum::{extract::{Path, State}, Json};

use crate::api::models::{PasswordResetResponse, PasswordResetRequestResponse};
use crate::api::requests::{
    RequestPasswordResetRequest, ConfirmPasswordResetRequest, ChangePasswordRequest,
    TeacherApproveResetRequest, TeacherRejectResetRequest, TeacherSetPasswordRequest,
};
use crate::state::AppState;
use crate::auth::AuthContext;
use crate::auth::permissions::ensure_teacher;
use crate::infra::errors::AppResult;
use crate::utils::parse_id;

pub async fn request_password_reset(
    State(state): State<AppState>,
    Json(req): Json<RequestPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    state.password_reset_service
        .request_reset_via_email(&req.email)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "If an account with that email exists, a password reset code has been sent.".to_string(),
    }))
}

pub async fn confirm_password_reset(
    State(state): State<AppState>,
    Json(req): Json<ConfirmPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    tracing::info!("Password reset confirmation request for email: {}, code length: {}", req.email, req.code.len());
    
    state.password_reset_service
        .confirm_reset_and_change_password(&req.email, &req.code, &req.new_password)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Password has been reset successfully. You can now login with your new password.".to_string(),
    }))
}

pub async fn change_password_authenticated(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    state.password_reset_service
        .change_password_authenticated(auth.user_id, &req.old_password, &req.new_password)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Password changed successfully.".to_string(),
    }))
}

pub async fn request_password_reset_via_teacher(
    State(state): State<AppState>,
    Json(req): Json<RequestPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    state.password_reset_service
        .request_reset_via_teacher(&req.email)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Your request has been sent to teachers for approval.".to_string(),
    }))
}

pub async fn get_pending_teacher_requests(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<PasswordResetRequestResponse>>> {
    ensure_teacher(&auth).await?;

    let requests = state.password_reset_service
        .get_pending_teacher_requests()
        .await?;

    Ok(Json(requests.into_iter().map(Into::into).collect()))
}

pub async fn teacher_approve_reset(
    auth: AuthContext,
    Path(request_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherApproveResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    ensure_teacher(&auth).await?;

    let request_id = parse_id(&request_id)?;
    state.password_reset_service
        .teacher_approve_and_set_password(request_id, auth.user_id, &req.new_password, req.notes)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Password reset approved and password set successfully.".to_string(),
    }))
}

pub async fn teacher_reject_reset(
    auth: AuthContext,
    Path(request_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherRejectResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    ensure_teacher(&auth).await?;

    let request_id = parse_id(&request_id)?;
    state.password_reset_service
        .teacher_reject_request(request_id, auth.user_id, req.reason)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Password reset request rejected.".to_string(),
    }))
}

pub async fn teacher_set_student_password(
    auth: AuthContext,
    Path(student_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherSetPasswordRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    ensure_teacher(&auth).await?;

    let student_id = parse_id(&student_id)?;
    state.password_reset_service
        .teacher_set_student_password(student_id, auth.user_id, &req.new_password)
        .await?;

    Ok(Json(PasswordResetResponse {
        message: "Student password set successfully.".to_string(),
    }))
}


