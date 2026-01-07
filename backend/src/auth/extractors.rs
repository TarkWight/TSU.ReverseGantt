use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

use crate::state::AppState;
use crate::domain::GlobalRole;
use crate::infra::errors::AppError;
use crate::infra::security::decode_token;
use crate::utils::{Id, parse_id};

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user_id: Id,
    pub is_teacher: bool,
    pub email: String,
    pub name: String,
}

impl FromRequestParts<AppState> for AuthContext {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".to_string()))?;

        let claims = decode_token(token)?;
        let user_id = parse_id(&claims.sub)?;

        let user = state.user_service.get_by_id(user_id).await?;

        let is_teacher = user.global_role == GlobalRole::Teacher;

        Ok(AuthContext {
            user_id,
            is_teacher,
            email: user.email,
            name: user.name,
        })
    }
}
