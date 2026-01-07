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
        let row = sqlx::query!(
            r#"
            SELECT
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                progress, buffer, hardness,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch task")?;

        Ok(row.map(|r| {
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
        }))
    }

    async fn insert(&self, task: &Task) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO tasks (
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                progress, buffer, hardness,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            "#,
            task.id,
            task.project_id,
            task.parent_task_id,
            task.name,
            task.description,
            task.task_type.to_string(),
            task.status.to_string(),
            task.priority.to_string(),
            task.estimated_duration,
            task.progress,
            task.buffer,
            task.hardness.to_string(),
            task.schedule.ls,
            task.schedule.lf,
            task.schedule.slack,
            task.schedule.is_critical,
            task.created_at,
            task.updated_at
        )
            .execute(&self.pool)
            .await
            .context("Failed to insert task")?;

        Ok(())
    }

    async fn update(&self, task: &Task) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE tasks SET
                name = $2, description = $3, task_type = $4, status = $5,
                priority = $6, estimated_duration = $7, progress = $8,
                buffer = $9, hardness = $10, updated_at = $11
            WHERE id = $1
            "#,
            task.id,
            task.name,
            task.description,
            task.task_type.to_string(),
            task.status.to_string(),
            task.priority.to_string(),
            task.estimated_duration,
            task.progress,
            task.buffer,
            task.hardness.to_string(),
            task.updated_at
        )
            .execute(&self.pool)
            .await
            .context("Failed to update task")?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .context("Failed to delete task")?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_schedule(&self, id: Id, schedule: &Schedule) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE tasks SET
                schedule_ls = $2, schedule_lf = $3, schedule_slack = $4, schedule_is_critical = $5,
                updated_at = NOW()
            WHERE id = $1
            "#,
            id,
            schedule.ls,
            schedule.lf,
            schedule.slack,
            schedule.is_critical
        )
            .execute(&self.pool)
            .await
            .context("Failed to update task schedule")?;

        Ok(result.rows_affected() > 0)
    }

    async fn has_not_done_descendants(&self, task_id: Id) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id, status, parent_task_id
                FROM tasks
                WHERE parent_task_id = $1

                UNION ALL

                SELECT t.id, t.status, t.parent_task_id
                FROM tasks t
                INNER JOIN descendants d ON t.parent_task_id = d.id
            )
            SELECT COUNT(*) as count
            FROM descendants
            WHERE status != 'Done'
            "#,
            task_id
        )
            .fetch_one(&self.pool)
            .await
            .context("Failed to check descendants")?;

        Ok(result.count.unwrap_or(0) > 0)
    }
}

