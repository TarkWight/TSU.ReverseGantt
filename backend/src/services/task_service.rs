use async_trait::async_trait;
use crate::domain::Task;
use crate::utils::{AppResult, Id};

#[async_trait]
pub trait TaskService: Send + Sync {
    async fn get_by_project(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn get_by_id(&self, id: Id) -> AppResult<Task>;
    async fn create(&self, task: Task) -> AppResult<Task>;
    async fn update(&self, id: Id, task: Task) -> AppResult<Task>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}