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
    async fn get_by_task(&self, _task_id: Id) -> AppResult<Option<Review>> {
        Err(AppError::Internal(anyhow::anyhow!(
            "get_by_task not implemented yet"
        )))
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