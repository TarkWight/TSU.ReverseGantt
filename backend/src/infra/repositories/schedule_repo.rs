use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;
use chrono::NaiveDate;

use crate::domain::{Task, Dependency, Schedule, TaskType, TaskStatus, Priority, Hardness, DepType};
use crate::utils::Id;

#[async_trait]
pub trait ScheduleRepository: Send + Sync {
    async fn get_project_dates(&self, project_id: Id) -> anyhow::Result<Option<(Option<NaiveDate>, NaiveDate)>>;
    async fn get_tasks_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Task>>;
    async fn get_dependencies_by_task_ids(&self, task_ids: &[Id]) -> anyhow::Result<Vec<Dependency>>;
    async fn update_task_schedule(&self, task: &Task) -> anyhow::Result<()>;
}

pub struct PgScheduleRepository {
    pool: PgPool,
}

impl PgScheduleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ScheduleRepository for PgScheduleRepository {
    async fn get_project_dates(&self, project_id: Id) -> anyhow::Result<Option<(Option<NaiveDate>, NaiveDate)>> {
        let row = sqlx::query!(
            "SELECT start_date, due_date FROM projects WHERE id = $1",
            project_id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch project")?;

        Ok(row.map(|r| (r.start_date, r.due_date)))
    }


    async fn get_tasks_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Task>> {
        todo!()
    }

    async fn get_dependencies_by_task_ids(&self, task_ids: &[Id]) -> anyhow::Result<Vec<Dependency>> {
        todo!()
    }

    async fn update_task_schedule(&self, task: &Task) -> anyhow::Result<()> {
        todo!()
    }
}

