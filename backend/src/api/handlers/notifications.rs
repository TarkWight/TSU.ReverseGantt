use axum::{
    extract::{
        Path,
        Query,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::infra::errors::AppResult;
use crate::utils::parse_id;
use crate::auth::AuthContext;
use crate::state::AppState;
use crate::api::requests::GetNotificationsQuery;
use crate::api::models::NotificationsListResponse;

pub async fn get_notifications(
    auth: AuthContext,
    Query(query): Query<GetNotificationsQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<NotificationsListResponse>> {
    let notifications = state
        .notification_service
        .get_by_user(auth.user_id, query.limit)
        .await?;

    let unread_count = state
        .notification_service
        .get_unread_count(auth.user_id)
        .await?;

    Ok(Json(NotificationsListResponse {
        notifications: notifications
            .into_iter()
            .map(Into::into)
            .collect(),
        unread_count,
    }))
}

pub async fn get_unread_count(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    let count = state
        .notification_service
        .get_unread_count(auth.user_id)
        .await?;

    Ok(Json(serde_json::json!({ "unreadCount": count })))
}

pub async fn mark_as_read(
    auth: AuthContext,
    Path(notification_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let id = parse_id(&notification_id)?;

    state
        .notification_service
        .mark_as_read(id, auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn mark_all_as_read(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    state
        .notification_service
        .mark_all_as_read(auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

