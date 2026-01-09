use axum::Router;
use tower_http::cors::CorsLayer;

use crate::routes;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::health::router())
        .merge(routes::auth::router(state.clone()))
        .merge(routes::users::router(state.clone()))
        .merge(routes::projects::router(state.clone()))
        .merge(routes::tasks::router(state.clone()))
        .merge(routes::memberships::router(state.clone()))
        .merge(routes::assignments::router(state.clone()))
        .merge(routes::notifications::router(state.clone()))
        .merge(routes::password_reset::router(state.clone()))
        .merge(routes::export::router(state.clone()))
        .layer(CorsLayer::permissive())
}

