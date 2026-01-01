use std::sync::Arc;
use std::time::Duration;

pub async fn login(
    State(user_service): State<Arc<dyn UserService>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user = user_service.authenticate(&req.email, &req.password).await?;

    // 24 hours TTL
    let token = generate_token(user.id, Duration::from_secs(24 * 60 * 60))?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id.to_string(),
    }))
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