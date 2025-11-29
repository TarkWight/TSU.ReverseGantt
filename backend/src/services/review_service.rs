use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::Review;
use crate::utils::{AppResult, Id};
use crate::utils::AppError;

#[async_trait]
pub trait ReviewService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Option<Review>>;
    async fn create(&self, review: Review) -> AppResult<Review>;
    async fn update(&self, id: Id, review: Review) -> AppResult<Review>;
}

pub struct ReviewServiceImpl {
    pool: PgPool,
}

impl ReviewServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewService for ReviewServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Option<Review>> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                task_id,
                reviewer_id,
                decision,
                comment,
                created_at,
                updated_at
            FROM reviews
            WHERE task_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            task_id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(Review {
                id: row.id,
                task_id: row.task_id,
                reviewer_id: row.reviewer_id,
                decision: row.decision.and_then(|d| d.parse().ok()),
                comment: row.comment,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, _review: Review) -> AppResult<Review> {
        Err(AppError::Internal(anyhow::anyhow!(
            "create not implemented yet"
        )))
    }

    async fn update(&self, _id: Id, _review: Review) -> AppResult<Review> {
        Err(AppError::Internal(anyhow::anyhow!(
            "update not implemented yet"
        )))
    }
}