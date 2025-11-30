use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand_core::OsRng;

use crate::utils::AppError;

fn password_pepper() -> String {
    std::env::var("PASSWORD_PEPPER").unwrap_or_else(|_| "".to_string())
}

pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let pepper = password_pepper();
    let to_hash = format!("{plain}{pepper}");

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(to_hash.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hash error: {e}")))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, plain: &str) -> Result<bool, AppError> {
    let pepper = password_pepper();
    let to_verify = format!("{plain}{pepper}");

    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid password hash format: {e}")))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(to_verify.as_bytes(), &parsed_hash)
        .is_ok())
}