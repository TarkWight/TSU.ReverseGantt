/* TODO: - refine error handling
* sqlx::Error -> AppError::Internal (From)
* ->!map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
*/
use crate::utils::{AppError, AppResult, Id};
use crate::utils::validate_project_name;
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
        let row = sqlx::query!(
        r#"
        SELECT
            id, name, description,
            start_date, due_date,
            created_at, updated_at
        FROM projects
        WHERE id = $1
        "#,
        id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
            .ok_or_else(|| AppError::NotFound(format!("Project with id {} not found", id)))?;

        Ok(Project {
            id: row.id,
            name: row.name,
            description: row.description,
            start_date: row.start_date,
            due_date: row.due_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    async fn create(&self, project: Project) -> AppResult<Project> {
        validate_project_name(&project.name)?;

        sqlx::query!(
        r#"
        INSERT INTO projects
            (id, name, description, start_date, due_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        project.id,
        project.name,
        project.description,
        project.start_date,
        project.due_date,
        project.created_at,
        project.updated_at
    )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

        Ok(project)
    }

    async fn update(&self, id: Id, project: Project) -> AppResult<Project> {
        validate_project_name(&project.name)?;

        let rows = sqlx::query!(
        r#"
        UPDATE projects
        SET name = $2,
            description = $3,
            start_date = $4,
            due_date = $5,
            updated_at = $6
        WHERE id = $1
        "#,
        id,
        project.name,
        project.description,
        project.start_date,
        project.due_date,
        project.updated_at
    )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
            .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Project with id {} not found", id)));
        }

        Ok(project)
    }

    async fn delete(&self, id: Id) -> AppResult<()> {
        let rows = sqlx::query!(
        "DELETE FROM projects WHERE id = $1",
        id
    )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
            .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Project with id {} not found", id)));
        }

        Ok(())
    }
}