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
        notifications: notifications.into_iter().map(Into::into).collect(),
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
