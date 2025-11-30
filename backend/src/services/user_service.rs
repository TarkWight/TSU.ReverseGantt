use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::User;
use crate::utils::{hash_password, AppError, AppResult};
use crate::utils::auth::verify_password;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn authenticate(&self, email: &str, password: &str) -> AppResult<User>;
}

pub struct UserServiceImpl {
    pool: PgPool,
}

impl UserServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn authenticate(&self, email: &str, password: &str) -> AppResult<User> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                email,
                name,
                password_hash,
                created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {e}")))?;

        let row = row.ok_or_else(|| {
            AppError::Validation(format!("Invalid credentials for email {}", email))
        })?;

        let hash = row.password_hash;

        if hash.is_empty() {
            return Err(AppError::Internal(anyhow::anyhow!("User has no password hash set")));
        }

        if !verify_password(&hash, &password)? {
            return Err(AppError::Validation("Invalid email or password".into()));
        }

        Ok(User {
            id: row.id,
            email: row.email,
            name: row.name,
            created_at: row.created_at,
        })
    }
}