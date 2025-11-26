use async_trait::async_trait;
use crate::domain::{Task, Schedule};
use crate::utils::{AppResult, Id};

#[async_trait]
pub trait ScheduleService: Send + Sync {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>>;
}