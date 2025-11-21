use crate::utils::{AppError, AppResult, Id};
use crate::domain::Project;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn get_all(&self) -> AppResult<Vec<Project>>;
    async fn get_by_id(&self, id: Id) -> AppResult<Project>;
    async fn create(&self, project: Project) -> AppResult<Project>;
    async fn update(&self, id: Id, project: Project) -> AppResult<Project>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}
pub struct ProjectServiceImpl {
    pool: PgPool,
}

impl ProjectServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn get_all(&self) -> AppResult<Vec<Project>> {
        let rows = sqlx::query!(
        r#"
        SELECT
            id, name, description,
            start_date, due_date,
            created_at, updated_at
        FROM projects
        ORDER BY created_at DESC
        "#
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

        let projects = rows.into_iter().map(|row| Project {
            id: row.id,
            name: row.name,
            description: row.description,
            start_date: row.start_date,
            due_date: row.due_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(projects)
    }

    async fn get_by_id(&self, id: Id) -> AppResult<Project> {
        todo!()
    }

    async fn create(&self, project: Project) -> AppResult<Project> {
        todo!()
    }

    async fn update(&self, id: Id, project: Project) -> AppResult<Project> {
        todo!()
    }

    async fn delete(&self, id: Id) -> AppResult<()> {
        todo!()
    }
}