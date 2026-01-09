use axum::{routing::post, Json, Router};
use axum::extract::State;

use crate::state::AppState;
use crate::api::handlers::users::{login, register};
use crate::api::models::LoginResponse;
use crate::api::requests::{LoginRequest, RegisterRequest};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .with_state(state)
}

async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    login(State(state), Json(req)).await
}

async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<LoginResponse>> {
    register(State(state), Json(req)).await
}
