use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand_core::OsRng;

use crate::infra::errors::{AppError, AppResult};

fn password_pepper() -> String {
    std::env::var("PASSWORD_PEPPER").unwrap_or_default()
}

pub fn hash_password(plain: &str) -> AppResult<String> {
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

pub fn verify_password(hash: &str, plain: &str) -> AppResult<bool> {
    let pepper = password_pepper();
    let to_verify = format!("{plain}{pepper}");

    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid password hash format: {e}")))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(to_verify.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn validate_password(password: &str) -> AppResult<()> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters long".into(),
        ));
    }

    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());

    if !has_lower {
        return Err(AppError::Validation(
            "Password must contain at least one lowercase letter".into(),
        ));
    }
    if !has_upper {
        return Err(AppError::Validation(
            "Password must contain at least one uppercase letter".into(),
        ));
    }
    if !has_digit {
        return Err(AppError::Validation(
            "Password must contain at least one digit".into(),
        ));
    }
    if !has_special {
        return Err(AppError::Validation(
            "Password must contain at least one special character".into(),
        ));
    }

    Ok(())
}

