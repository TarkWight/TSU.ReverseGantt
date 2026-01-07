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
        let rows = sqlx::query!(
            r#"
            SELECT
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                progress, buffer, hardness,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            FROM tasks
            WHERE project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch tasks")?;

        Ok(rows
            .into_iter()
            .map(|r| Task {
                id: r.id,
                project_id: r.project_id,
                parent_task_id: r.parent_task_id,
                name: r.name,
                description: r.description,
                task_type: r.task_type.parse().unwrap_or(TaskType::Task),
                status: r.status.parse().unwrap_or(TaskStatus::Planned),
                priority: r.priority.parse().unwrap_or(Priority::Normal),
                estimated_duration: r.estimated_duration,
                progress: r.progress.unwrap_or(0),
                buffer: r.buffer.unwrap_or(0),
                hardness: r.hardness.parse().unwrap_or(Hardness::Soft),
                schedule: Schedule {
                    ls: r.schedule_ls,
                    lf: r.schedule_lf,
                    slack: r.schedule_slack,
                    is_critical: r.schedule_is_critical,
                },
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }


    async fn get_dependencies_by_task_ids(&self, task_ids: &[Id]) -> anyhow::Result<Vec<Dependency>> {
        todo!()
    }

    async fn update_task_schedule(&self, task: &Task) -> anyhow::Result<()> {
        todo!()
    }
}

