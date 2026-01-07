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
        todo!()
    }

    async fn insert(&self, review: &Review) -> anyhow::Result<()> {
        todo!()
    }

    async fn update(&self, review: &Review) -> anyhow::Result<bool> {
        todo!()
    }
}

