use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::{AppError, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string())
}

pub fn generate_token(user_id: Id, ttl: Duration) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Time error: {e}")))?;

    let exp = now + ttl;

    let claims = Claims {
        sub: user_id.to_string(),
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

pub fn decode_token(token: &str) -> Result<Claims, AppError> {
    let secret = jwt_secret();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|e| AppError::BadRequest(format!("Invalid token: {e}")))?;

    Ok(data.claims)
}