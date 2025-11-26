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

    async fn get_by_id(&self, id: Id) -> AppResult<Task> {
        let row = query!(
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
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;

        Ok(Task {
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
    }

    async fn create(&self, task: Task) -> AppResult<Task> {
        query!(
            r#"
            INSERT INTO tasks (
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                planned_start, planned_finish, actual_start, actual_finish,
                progress, buffer, hardness, deadline,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9,
                $10, $11, $12, $13,
                $14, $15, $16, $17,
                $18, $19, $20, $21,
                $22, $23
            )
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
            task.planned_start,
            task.planned_finish,
            task.actual_start,
            task.actual_finish,
            task.progress,
            task.buffer,
            task.hardness.to_string(),
            task.deadline,
            task.schedule.ls,
            task.schedule.lf,
            task.schedule.slack,
            task.schedule.is_critical,
            task.created_at,
            task.updated_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        Ok(task)
    }

    async fn update(&self, id: Id, task: Task) -> AppResult<Task> {
        let rows_affected = query!(
            r#"
            UPDATE tasks
            SET
                name = $2,
                description = $3,
                task_type = $4,
                status = $5,
                priority = $6,
                estimated_duration = $7,
                planned_start = $8,
                planned_finish = $9,
                actual_start = $10,
                actual_finish = $11,
                progress = $12,
                buffer = $13,
                hardness = $14,
                deadline = $15,
                schedule_ls = $16,
                schedule_lf = $17,
                schedule_slack = $18,
                schedule_is_critical = $19,
                updated_at = $20
            WHERE id = $1
            "#,
            id,
            task.name,
            task.description,
            task.task_type.to_string(),
            task.status.to_string(),
            task.priority.to_string(),
            task.estimated_duration,
            task.planned_start,
            task.planned_finish,
            task.actual_start,
            task.actual_finish,
            task.progress,
            task.buffer,
            task.hardness.to_string(),
            task.deadline,
            task.schedule.ls,
            task.schedule.lf,
            task.schedule.slack,
            task.schedule.is_critical,
            task.updated_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", id)));
        }

        Ok(task)
    }

    async fn delete(&self, _id: Id) -> AppResult<()> {
        todo!("delete not implemented yet");
    }
}