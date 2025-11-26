use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query;

use crate::domain::{Dependency, DepType};
use crate::utils::{AppError, AppResult, Id};

#[async_trait]
pub trait DependencyService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>>;
    async fn create(&self, dependency: Dependency) -> AppResult<Dependency>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}

pub struct DependencyServiceImpl {
    pool: PgPool,
}

impl DependencyServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DependencyService for DependencyServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>> {
        let rows = query!(
            r#"
            SELECT
                id,
                from_task_id,
                to_task_id,
                dep_type,
                min_gap
            FROM dependencies
            WHERE from_task_id = $1 OR to_task_id = $1
            "#,
            task_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let dependencies = rows
            .into_iter()
            .map(|row| Dependency {
                id: row.id,
                from_task_id: row.from_task_id,
                to_task_id: row.to_task_id,
                dep_type: row
                    .dep_type
                    .parse()
                    .unwrap_or(DepType::FS),
                min_gap: row.min_gap,
            })
            .collect();

        Ok(dependencies)
    }

    async fn create(&self, _dependency: Dependency) -> AppResult<Dependency> {
        // will be implemented in a later commit
        todo!("create dependency not implemented yet");
    }

    async fn delete(&self, _id: Id) -> AppResult<()> {
        // will be implemented in a later commit
        todo!("delete dependency not implemented yet");
    }
}