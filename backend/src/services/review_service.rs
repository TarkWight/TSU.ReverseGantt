use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::Review;
use crate::utils::{AppResult, Id};

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