use async_trait::async_trait;
use sqlx::query;
use crate::utils::{AppResult, AppError, Id};
use crate::domain::{
    Task,
    Dependency,
    DepType,
    TaskType,
    TaskStatus,
    Priority,
    Schedule
};

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
    async fn get_tasks_with_dependencies(&self, project_id: Id)
                                         -> AppResult<(Vec<Task>, Vec<Dependency>)>
    {
        let tasks = query!(
            r#"
            SELECT
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                planned_start, planned_finish, actual_start, actual_finish,
                progress, buffer, hardness, deadline,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            FROM tasks
            WHERE project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let task_ids: Vec<Id> = tasks.iter().map(|t| t.id).collect();

        let deps = if !task_ids.is_empty() {
            query!(
                r#"
                SELECT id, from_task_id, to_task_id, dep_type, min_gap
                FROM dependencies
                WHERE from_task_id = ANY($1) OR to_task_id = ANY($1)
                "#,
                &task_ids[..]
            )
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
        } else {
            vec![]
        };

        let domain_tasks = tasks.into_iter().map(|row| {
            Task {
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
                hardness: row.hardness.parse().unwrap_or(crate::domain::enums::Hardness::Soft),
                deadline: row.deadline,
                schedule: Schedule {
                    ls: row.schedule_ls,
                    lf: row.schedule_lf,
                    slack: row.schedule_slack,
                    is_critical: row.schedule_is_critical,
                },
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        let domain_deps = deps.into_iter().map(|row| Dependency {
            id: row.id,
            from_task_id: row.from_task_id,
            to_task_id: row.to_task_id,
            dep_type: row.dep_type.parse().unwrap_or(DepType::FS),
            min_gap: row.min_gap,
        }).collect();

        Ok((domain_tasks, domain_deps))
    }
}