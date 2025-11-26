use async_trait::async_trait;
use sqlx::PgPool;
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

pub struct TaskServiceImpl {
    pool: PgPool,
}

impl TaskServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn get_by_project(&self, _project_id: Id) -> AppResult<Vec<Task>> {
        todo!("get_by_project not implemented yet");
    }

    async fn get_by_id(&self, _id: Id) -> AppResult<Task> {
        todo!("get_by_id not implemented yet");
    }

    async fn create(&self, _task: Task) -> AppResult<Task> {
        todo!("create not implemented yet");
    }

    async fn update(&self, _id: Id, _task: Task) -> AppResult<Task> {
        todo!("update not implemented yet");
    }

    async fn delete(&self, _id: Id) -> AppResult<()> {
        todo!("delete not implemented yet");
    }
}