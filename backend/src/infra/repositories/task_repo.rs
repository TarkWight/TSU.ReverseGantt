use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{Task, Schedule, TaskType, TaskStatus, Priority, Hardness};
use crate::utils::Id;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Task>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Task>>;
    async fn insert(&self, task: &Task) -> anyhow::Result<()>;
    async fn update(&self, task: &Task) -> anyhow::Result<bool>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
    async fn update_schedule(&self, id: Id, schedule: &Schedule) -> anyhow::Result<bool>;
    async fn has_not_done_descendants(&self, task_id: Id) -> anyhow::Result<bool>;
}

pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row_to_task(
        id: Id,
        project_id: Id,
        parent_task_id: Option<Id>,
        name: String,
        description: Option<String>,
        task_type: String,
        status: String,
        priority: String,
        estimated_duration: Option<i64>,
        progress: Option<i32>,
        buffer: Option<i64>,
        hardness: String,
        schedule_ls: Option<chrono::DateTime<chrono::Utc>>,
        schedule_lf: Option<chrono::DateTime<chrono::Utc>>,
        schedule_slack: Option<i64>,
        schedule_is_critical: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Task {
        Task {
            id,
            project_id,
            parent_task_id,
            name,
            description,
            task_type: task_type.parse().unwrap_or(TaskType::Task),
            status: status.parse().unwrap_or(TaskStatus::Planned),
            priority: priority.parse().unwrap_or(Priority::Normal),
            estimated_duration,
            progress: progress.unwrap_or(0),
            buffer: buffer.unwrap_or(0),
            hardness: hardness.parse().unwrap_or(Hardness::Soft),
            schedule: Schedule {
                ls: schedule_ls,
                lf: schedule_lf,
                slack: schedule_slack,
                is_critical: schedule_is_critical,
            },
            created_at,
            updated_at,
        }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Task>> {
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
            ORDER BY created_at
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch tasks")?;

        Ok(rows
            .into_iter()
            .map(|r| {
                Self::map_row_to_task(
                    r.id,
                    r.project_id,
                    r.parent_task_id,
                    r.name,
                    r.description,
                    r.task_type,
                    r.status,
                    r.priority,
                    r.estimated_duration,
                    r.progress,
                    r.buffer,
                    r.hardness,
                    r.schedule_ls,
                    r.schedule_lf,
                    r.schedule_slack,
                    r.schedule_is_critical,
                    r.created_at,
                    r.updated_at,
                )
            })
            .collect())
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Task>> {
        todo!()
    }

    async fn insert(&self, task: &Task) -> anyhow::Result<()> {
        todo!()
    }

    async fn update(&self, task: &Task) -> anyhow::Result<bool> {
        todo!()
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn update_schedule(&self, id: Id, schedule: &Schedule) -> anyhow::Result<bool> {
        todo!()
    }

    async fn has_not_done_descendants(&self, task_id: Id) -> anyhow::Result<bool> {
        todo!()
    }
}

