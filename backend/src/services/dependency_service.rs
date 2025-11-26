use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::Dependency;
use crate::utils::{AppResult, Id};

#[async_trait]
pub trait DependencyService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>>;
    async fn create(&self, dependency: Dependency) -> AppResult<Dependency>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}

pub struct DependencyServiceImpl {
    pool: PgPool,
}

impl DependencyServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}