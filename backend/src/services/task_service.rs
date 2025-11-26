use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{Task, TaskStatus, TaskType, Priority, Schedule};
use crate::utils::{AppError, AppResult, Id};
use sqlx::query;

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
    async fn get_by_project(&self, project_id: Id) -> AppResult<Vec<Task>> {
        let rows = query!(
            r#"
            SELECT
                id,
                project_id,
                parent_task_id,
                name,
                description,
                task_type,
                status,
                priority,
                estimated_duration,
                planned_start,
                planned_finish,
                actual_start,
                actual_finish,
                progress,
                buffer,
                hardness,
                deadline,
                schedule_ls,
                schedule_lf,
                schedule_slack,
                schedule_is_critical,
                created_at,
                updated_at
            FROM tasks
            WHERE project_id = $1
            ORDER BY created_at ASC
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let tasks = rows
            .into_iter()
            .map(|row| Task {
                id: row.id,
                project_id: row.project_id,
                parent_task_id: row.parent_task_id,
                name: row.name,
                description: row.description,
                task_type: row.task_type.parse().unwrap_or(TaskType::Task),
                status: row.status.parse().unwrap_or(TaskStatus::Planned),
                priority: row.priority.parse().unwrap_or(Priority::Normal),
                estimated_duration: row.estimated_duration,
                planned_start: row.planned_start,
                planned_finish: row.planned_finish,
                actual_start: row.actual_start,
                actual_finish: row.actual_finish,
                progress: row.progress.unwrap_or(0),
                buffer: row.buffer.unwrap_or(0),
                hardness: row
                    .hardness
                    .parse()
                    .unwrap_or(crate::domain::enums::Hardness::Soft),
                deadline: row.deadline,
                schedule: Schedule {
                    ls: row.schedule_ls,
                    lf: row.schedule_lf,
                    slack: row.schedule_slack,
                    is_critical: row.schedule_is_critical,
                },
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(tasks)
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