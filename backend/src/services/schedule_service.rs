use async_trait::async_trait;
use crate::domain::{Task, Schedule};
use crate::utils::{AppResult, Id};

#[async_trait]
pub trait ScheduleService: Send + Sync {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>>;
}

use sqlx::PgPool;

pub struct ScheduleServiceImpl {
    pool: PgPool,
}

impl ScheduleServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}