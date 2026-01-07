use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::Review;
use crate::utils::Id;

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Option<Review>>;
    async fn insert(&self, review: &Review) -> anyhow::Result<()>;
    async fn update(&self, review: &Review) -> anyhow::Result<bool>;
}

pub struct PgReviewRepository {
    pool: PgPool,
}

impl PgReviewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for PgReviewRepository {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Option<Review>> {
        let row = sqlx::query!(
            r#"
            SELECT id, task_id, reviewer_id, decision, comment, created_at, updated_at
            FROM reviews
            WHERE task_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            task_id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch review")?;

        Ok(row.map(|r| Review {
            id: r.id,
            task_id: r.task_id,
            reviewer_id: r.reviewer_id,
            decision: r.decision.and_then(|d| d.parse().ok()),
            comment: r.comment,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn insert(&self, review: &Review) -> anyhow::Result<()> {
        todo!()
    }

    async fn update(&self, review: &Review) -> anyhow::Result<bool> {
        todo!()
    }
}

