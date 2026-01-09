use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;
use chrono::Utc;

use crate::domain::{PasswordResetCode, ResetRequestType, ResetRequestStatus};
use crate::utils::Id;

#[async_trait]
pub trait PasswordResetRepository: Send + Sync {
    async fn insert(&self, reset: &PasswordResetCode) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<PasswordResetCode>>;
    async fn find_pending_by_user(&self, user_id: Id, request_type: ResetRequestType) -> anyhow::Result<Option<PasswordResetCode>>;
    async fn find_all_teacher_requests_pending(&self) -> anyhow::Result<Vec<(PasswordResetCode, String, String)>>; // (reset, user_name, user_email)
    async fn update_status(&self, id: Id, status: ResetRequestStatus, reviewed_by: Option<Id>, notes: Option<&str>) -> anyhow::Result<bool>;
    async fn expire_old_codes(&self) -> anyhow::Result<u64>;
}

pub struct PgPasswordResetRepository {
    pool: PgPool,
}

impl PgPasswordResetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_request_type(s: &str) -> ResetRequestType {
        s.parse().unwrap_or(ResetRequestType::Email)
    }

    fn parse_status(s: &str) -> ResetRequestStatus {
        s.parse().unwrap_or(ResetRequestStatus::Pending)
    }
}

#[async_trait]
impl PasswordResetRepository for PgPasswordResetRepository {
    async fn insert(&self, reset: &PasswordResetCode) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO password_reset_codes (id, user_id, code_hash, request_type, status, expires_at, created_at, reviewed_by, reviewed_at, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            reset.id,
            reset.user_id,
            reset.code_hash,
            reset.request_type.to_string(),
            reset.status.to_string(),
            reset.expires_at,
            reset.created_at,
            reset.reviewed_by,
            reset.reviewed_at,
            reset.notes
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert password reset code")?;

        Ok(())
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<PasswordResetCode>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, code_hash, request_type, status, expires_at, created_at, reviewed_by, reviewed_at, notes
            FROM password_reset_codes WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch password reset code")?;

        Ok(row.map(|r| PasswordResetCode {
            id: r.id,
            user_id: r.user_id,
            code_hash: r.code_hash,
            request_type: Self::parse_request_type(&r.request_type),
            status: Self::parse_status(&r.status),
            expires_at: r.expires_at,
            created_at: r.created_at,
            reviewed_by: r.reviewed_by,
            reviewed_at: r.reviewed_at,
            notes: r.notes,
        }))
    }

    async fn find_pending_by_user(&self, user_id: Id, request_type: ResetRequestType) -> anyhow::Result<Option<PasswordResetCode>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, code_hash, request_type, status, expires_at, created_at, reviewed_by, reviewed_at, notes
            FROM password_reset_codes
            WHERE user_id = $1 AND request_type = $2 AND status = 'pending'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id,
            request_type.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch pending password reset code")?;

        Ok(row.map(|r| PasswordResetCode {
            id: r.id,
            user_id: r.user_id,
            code_hash: r.code_hash,
            request_type: Self::parse_request_type(&r.request_type),
            status: Self::parse_status(&r.status),
            expires_at: r.expires_at,
            created_at: r.created_at,
            reviewed_by: r.reviewed_by,
            reviewed_at: r.reviewed_at,
            notes: r.notes,
        }))
    }

    async fn find_all_teacher_requests_pending(&self) -> anyhow::Result<Vec<(PasswordResetCode, String, String)>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                prc.id, prc.user_id, prc.code_hash, prc.request_type, prc.status, prc.expires_at, prc.created_at, prc.reviewed_by, prc.reviewed_at, prc.notes,
                u.name as user_name, u.email as user_email
            FROM password_reset_codes prc
            JOIN users u ON prc.user_id = u.id
            WHERE prc.request_type = 'teacher_request' AND prc.status = 'pending'
            ORDER BY prc.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch teacher password reset requests")?;

        Ok(rows
            .into_iter()
            .map(|r| {
                (
                    PasswordResetCode {
                        id: r.id,
                        user_id: r.user_id,
                        code_hash: r.code_hash,
                        request_type: Self::parse_request_type(&r.request_type),
                        status: Self::parse_status(&r.status),
                        expires_at: r.expires_at,
                        created_at: r.created_at,
                        reviewed_by: r.reviewed_by,
                        reviewed_at: r.reviewed_at,
                        notes: r.notes,
                    },
                    r.user_name,
                    r.user_email,
                )
            })
            .collect())
    }

    async fn update_status(&self, id: Id, status: ResetRequestStatus, reviewed_by: Option<Id>, notes: Option<&str>) -> anyhow::Result<bool> {
        let now = Utc::now();
        let result = sqlx::query!(
            r#"
            UPDATE password_reset_codes
            SET status = $2, reviewed_by = $3, reviewed_at = $4, notes = $5
            WHERE id = $1
            "#,
            id,
            status.to_string(),
            reviewed_by,
            if reviewed_by.is_some() { Some(now) } else { None },
            notes
        )
        .execute(&self.pool)
        .await
        .context("Failed to update password reset code status")?;

        Ok(result.rows_affected() > 0)
    }

    async fn expire_old_codes(&self) -> anyhow::Result<u64> {
        let now = Utc::now();
        let result = sqlx::query!(
            "UPDATE password_reset_codes SET status = 'expired' WHERE status = 'pending' AND expires_at IS NOT NULL AND expires_at < $1",
            now
        )
        .execute(&self.pool)
        .await
        .context("Failed to expire old password reset codes")?;

        Ok(result.rows_affected())
    }
}


