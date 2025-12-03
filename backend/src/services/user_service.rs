use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::User;
use crate::utils::{hash_password, AppError, AppResult};
use crate::utils::auth::{validate_password, verify_password};
use crate::utils::{generate_token};
use chrono::Utc;
#[async_trait]
pub trait UserService: Send + Sync {
    async fn authenticate(&self, email: &str, password: &str) -> AppResult<User>;
    async fn register(&self, email: String, name: String, password: String) -> AppResult<(User, String)>;

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

    async fn register(&self, email: String, name: String, password: String) -> AppResult<(User, String)> {
        validate_password(&password)?;

        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE email = $1
            ) AS "exists!"
            "#,
            email
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {e}")))?;

        if exists {
            return Err(AppError::Validation(format!("User with email {} already exists", email)));
        }

        let password_hash = hash_password(&password)?;

        let id = crate::utils::generate_id();
        let now = Utc::now();

        let user = User {
            id,
            email: email.clone(),
            name,
            created_at: now,
        };

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, name, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.email,
            user.name,
            password_hash,
            user.created_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {e}")))?;

        // TTL 24 hours
        let token = generate_token(user.id, std::time::Duration::from_secs(86400))?;

        Ok((user, token))
    }
}