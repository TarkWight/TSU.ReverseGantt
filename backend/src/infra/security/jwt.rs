use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::infra::errors::{AppError, AppResult};
use crate::utils::Id;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub name: String,
    pub exp: usize,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string())
}

pub fn generate_token(
    user_id: Id,
    global_role: &str,
    name: String,
    ttl: Duration
) -> AppResult<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Time error: {e}")))?;

    let exp = now + ttl;

    let claims = Claims {
        sub: user_id.to_string(),
        role: global_role.to_string(),
        name,
        exp: exp.as_secs() as usize,
    };

    let secret = jwt_secret();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Token encode error: {e}")))
}

pub fn decode_token(token: &str) -> AppResult<Claims> {
    let secret = jwt_secret();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))?;

    Ok(data.claims)
}
