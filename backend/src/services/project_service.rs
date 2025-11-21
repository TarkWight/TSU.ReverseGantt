use crate::utils::{AppResult, Id};
use crate::domain::Project;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn get_all(&self) -> AppResult<Vec<Project>>;
    async fn get_by_id(&self, id: Id) -> AppResult<Project>;
    async fn create(&self, project: Project) -> AppResult<Project>;
    async fn update(&self, id: Id, project: Project) -> AppResult<Project>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}
pub struct ProjectServiceImpl {
    pool: PgPool,
}

impl ProjectServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}