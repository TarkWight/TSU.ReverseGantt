use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{Dependency, DepType};
use crate::utils::Id;

#[async_trait]
pub trait DependencyRepository: Send + Sync {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Dependency>>;
    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Dependency>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Dependency>>;
    async fn insert(&self, dependency: &Dependency) -> anyhow::Result<()>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
    async fn get_task_project_id(&self, task_id: Id) -> anyhow::Result<Option<Id>>;
    async fn get_project_task_ids(&self, project_id: Id) -> anyhow::Result<Vec<Id>>;
}

pub struct PgDependencyRepository {
    pool: PgPool,
}

impl PgDependencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_dep_type(s: &str) -> DepType {
        s.parse().unwrap_or(DepType::FS)
    }
}

#[async_trait]
impl DependencyRepository for PgDependencyRepository {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Dependency>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, from_task_id, to_task_id, dep_type, min_gap
            FROM dependencies
            WHERE from_task_id = $1 OR to_task_id = $1
            "#,
            task_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch dependencies")?;

        Ok(rows
            .into_iter()
            .map(|r| Dependency {
                id: r.id,
                from_task_id: r.from_task_id,
                to_task_id: r.to_task_id,
                dep_type: Self::parse_dep_type(&r.dep_type),
                min_gap: r.min_gap,
            })
            .collect())
    }

    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Dependency>> {
        let rows = sqlx::query!(
            r#"
            SELECT d.id, d.from_task_id, d.to_task_id, d.dep_type, d.min_gap
            FROM dependencies d
            JOIN tasks t ON d.from_task_id = t.id
            WHERE t.project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch dependencies")?;

        Ok(rows
            .into_iter()
            .map(|r| Dependency {
                id: r.id,
                from_task_id: r.from_task_id,
                to_task_id: r.to_task_id,
                dep_type: Self::parse_dep_type(&r.dep_type),
                min_gap: r.min_gap,
            })
            .collect())
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Dependency>> {
        let row = sqlx::query!(
            r#"
            SELECT id, from_task_id, to_task_id, dep_type, min_gap
            FROM dependencies WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch dependency")?;

        Ok(row.map(|r| Dependency {
            id: r.id,
            from_task_id: r.from_task_id,
            to_task_id: r.to_task_id,
            dep_type: Self::parse_dep_type(&r.dep_type),
            min_gap: r.min_gap,
        }))
    }

    async fn insert(&self, dependency: &Dependency) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO dependencies (id, from_task_id, to_task_id, dep_type, min_gap)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            dependency.id,
            dependency.from_task_id,
            dependency.to_task_id,
            dependency.dep_type.to_string(),
            dependency.min_gap
        )
            .execute(&self.pool)
            .await
            .context("Failed to insert dependency")?;

        Ok(())
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        let result = sqlx::query!("DELETE FROM dependencies WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .context("Failed to delete dependency")?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_task_project_id(&self, task_id: Id) -> anyhow::Result<Option<Id>> {
        let row = sqlx::query!(
            "SELECT project_id FROM tasks WHERE id = $1",
            task_id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch task")?;

        Ok(row.map(|r| r.project_id))
    }

    async fn get_project_task_ids(&self, project_id: Id) -> anyhow::Result<Vec<Id>> {
        todo!()
    }
}

