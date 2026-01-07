use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::Review;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::ReviewRepository;
use crate::utils::Id;

#[async_trait]
pub trait ReviewService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Option<Review>>;
    async fn create(&self, review: Review) -> AppResult<Review>;
    async fn update(&self, id: Id, review: Review) -> AppResult<Review>;
}

pub struct ReviewServiceImpl {
    review_repo: Arc<dyn ReviewRepository>,
}

impl ReviewServiceImpl {
    pub fn new(review_repo: Arc<dyn ReviewRepository>) -> Self {
        Self { review_repo }
    }
}

#[async_trait]
impl ReviewService for ReviewServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Option<Review>> {
        self.review_repo
            .find_by_task(task_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn create(&self, review: Review) -> AppResult<Review> {
        self.review_repo
            .insert(&review)
            .await
            .map_err(AppError::Internal)?;

        Ok(review)
    }

    async fn update(&self, _id: Id, review: Review) -> AppResult<Review> {
        let updated = self.review_repo
            .update(&review)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("Review {} not found", review.id)));
        }

        Ok(review)
    }
}
