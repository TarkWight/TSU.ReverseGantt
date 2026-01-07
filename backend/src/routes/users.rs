use axum::{routing::{get, post, patch}, Json, Router};
use axum::extract::{Path, State};

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::handlers::users;
use crate::api::models::UserResponse;
use crate::api::requests::UpdateEmailNotificationsRequest;
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_all_users_handler))
        .route("/users/{id}", get(get_user_handler))
        .route("/users/{id}/promote-to-teacher", post(promote_to_teacher_handler))
        .route("/users/me/email-notifications", patch(update_email_notifications_handler))
        .with_state(state)
}

async fn get_all_users_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<UserResponse>>> {
    users::get_all_users(auth, State(state)).await
}

async fn get_user_handler(
    auth: AuthContext,
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    users::get_user(auth, Path(user_id), State(state)).await
}

async fn promote_to_teacher_handler(
    auth: AuthContext,
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    users::promote_to_teacher(auth, Path(user_id), State(state)).await
}

async fn update_email_notifications_handler(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<UpdateEmailNotificationsRequest>,
) -> AppResult<Json<UserResponse>> {
    users::update_email_notifications(auth, State(state), Json(req)).await
}
