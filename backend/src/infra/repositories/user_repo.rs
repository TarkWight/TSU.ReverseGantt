use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{User, GlobalRole};
use crate::utils::Id;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<(User, String)>>;
    async fn find_all(&self) -> anyhow::Result<Vec<User>>;
    async fn insert(&self, user: &User, password_hash: &str) -> anyhow::Result<()>;
    async fn update_global_role(&self, id: Id, role: GlobalRole) -> anyhow::Result<bool>;
    async fn update_email_notifications(&self, id: Id, enabled: bool) -> anyhow::Result<bool>;
    async fn update_password(&self, id: Id, password_hash: &str) -> anyhow::Result<bool>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_global_role(role: &str) -> GlobalRole {
        role
            .parse()
            .unwrap_or(GlobalRole::Student)
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, name, global_role, email_notifications_enabled
            FROM users WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch user")?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            name: r.name,
            global_role: Self::parse_global_role(&r.global_role),
            email_notifications_enabled: r.email_notifications_enabled,
        }))
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<(User, String)>> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, name, password_hash, global_role, email_notifications_enabled
            FROM users WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch user by email")?;

        Ok(row.map(|r| {
            (
                User {
                    id: r.id,
                    email: r.email,
                    name: r.name,
                    global_role: Self::parse_global_role(&r.global_role),
                    email_notifications_enabled: r.email_notifications_enabled,
                },
                r.password_hash,
            )
        }))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, email, name, global_role, email_notifications_enabled
            FROM users ORDER BY name, email
            "#
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch users")?;

        Ok(rows
            .into_iter()
            .map(|r| User {
                id: r.id,
                email: r.email,
                name: r.name,
                global_role: Self::parse_global_role(&r.global_role),
                email_notifications_enabled: r.email_notifications_enabled,
            })
            .collect())
    }

    async fn insert(&self, user: &User, password_hash: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, name, password_hash, global_role, email_notifications_enabled)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user.id,
            user.email,
            user.name,
            password_hash,
            user.global_role.to_string(),
            user.email_notifications_enabled
        )
            .execute(&self.pool)
            .await
            .context("Failed to insert user")?;

        Ok(())
    }

    async fn update_global_role(&self, id: Id, role: GlobalRole) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "UPDATE users SET global_role = $2 WHERE id = $1",
            id,
            role.to_string()
        )
            .execute(&self.pool)
            .await
            .context("Failed to update user role")?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_email_notifications(&self, id: Id, enabled: bool) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "UPDATE users SET email_notifications_enabled = $2 WHERE id = $1",
            id,
            enabled
        )
            .execute(&self.pool)
            .await
            .context("Failed to update email notifications")?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_password(&self, id: Id, password_hash: &str) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "UPDATE users SET password_hash = $2 WHERE id = $1",
            id,
            password_hash
        )
            .execute(&self.pool)
            .await
            .context("Failed to update password")?;

        Ok(result.rows_affected() > 0)
    }
}

