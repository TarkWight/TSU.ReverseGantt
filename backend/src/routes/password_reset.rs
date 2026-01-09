use axum::{routing::{get, post, patch}, Json, Router};
use axum::extract::{Path, State};

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::handlers::password_reset;
use crate::api::models::{PasswordResetResponse, PasswordResetRequestResponse};
use crate::api::requests::{
    RequestPasswordResetRequest, ConfirmPasswordResetRequest, ChangePasswordRequest,
    TeacherApproveResetRequest, TeacherRejectResetRequest, TeacherSetPasswordRequest,
};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/password-reset/request", post(request_password_reset_handler))
        .route("/password-reset/confirm", post(confirm_password_reset_handler))
        .route("/password-reset/change", patch(change_password_handler))
        .route("/password-reset/request-teacher", post(request_teacher_reset_handler))
        .route("/password-reset/teacher/requests", get(get_teacher_requests_handler))
        .route("/password-reset/teacher/requests/{id}/approve", post(approve_request_handler))
        .route("/password-reset/teacher/requests/{id}/reject", post(reject_request_handler))
        .route("/password-reset/teacher/students/{id}/set-password", post(set_student_password_handler))
        .with_state(state)
}

async fn request_password_reset_handler(
    State(state): State<AppState>,
    Json(req): Json<RequestPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::request_password_reset(State(state), Json(req)).await
}

async fn confirm_password_reset_handler(
    State(state): State<AppState>,
    Json(req): Json<ConfirmPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::confirm_password_reset(State(state), Json(req)).await
}

async fn change_password_handler(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::change_password_authenticated(auth, State(state), Json(req)).await
}

async fn request_teacher_reset_handler(
    State(state): State<AppState>,
    Json(req): Json<RequestPasswordResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::request_password_reset_via_teacher(State(state), Json(req)).await
}

async fn get_teacher_requests_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<PasswordResetRequestResponse>>> {
    password_reset::get_pending_teacher_requests(auth, State(state)).await
}

async fn approve_request_handler(
    auth: AuthContext,
    Path(request_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherApproveResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::teacher_approve_reset(auth, Path(request_id), State(state), Json(req)).await
}

async fn reject_request_handler(
    auth: AuthContext,
    Path(request_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherRejectResetRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::teacher_reject_reset(auth, Path(request_id), State(state), Json(req)).await
}

async fn set_student_password_handler(
    auth: AuthContext,
    Path(student_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<TeacherSetPasswordRequest>,
) -> AppResult<Json<PasswordResetResponse>> {
    password_reset::teacher_set_student_password(auth, Path(student_id), State(state), Json(req)).await
}


