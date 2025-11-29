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

    async fn create(&self, review: Review) -> AppResult<Review> {
        let reviewer_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE id = $1
            ) AS "exists!"
            "#,
            review.reviewer_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        if !reviewer_exists {
            return Err(AppError::Validation(format!(
                "Reviewer with id {} does not exist",
                review.reviewer_id
            )));
        }

        sqlx::query!(
            r#"
            INSERT INTO reviews (id, task_id, reviewer_id, decision, comment, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            review.id,
            review.task_id,
            review.reviewer_id,
            review.decision.map(|d| d.to_string()),
            review.comment,
            review.created_at,
            review.updated_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        Ok(review)
    }

    async fn update(&self, id: Id, review: Review) -> AppResult<Review> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE reviews
            SET decision = $2, comment = $3, updated_at = $4
            WHERE id = $1
            "#,
            id,
            review.decision.map(|d| d.to_string()),
            review.comment,
            review.updated_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Review with id {} not found", id)));
        }

        Ok(review)
    }
}