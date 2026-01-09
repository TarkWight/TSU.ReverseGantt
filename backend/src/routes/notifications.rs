use axum::{routing::{get, post}, Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::notifications::{
    GetNotificationsQuery, NotificationsListResponse,
    get_notifications, get_unread_count, mark_as_read, mark_all_as_read,
};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/notifications", get(get_notifications_handler))
        .route("/notifications/unread-count", get(get_unread_count_handler))
        .route("/notifications/{notification_id}/read", post(mark_read_handler))
        .route("/notifications/read-all", post(mark_all_read_handler))
        .with_state(state)
}

async fn get_notifications_handler(
    auth: AuthContext,
    query: Query<GetNotificationsQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<NotificationsListResponse>> {
    get_notifications(auth, query, State(state)).await
}

async fn get_unread_count_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    get_unread_count(auth, State(state)).await
}

async fn mark_read_handler(
    auth: AuthContext,
    Path(notification_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    mark_as_read(auth, Path(notification_id), State(state)).await
}

async fn mark_all_read_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    mark_all_as_read(auth, State(state)).await
}

