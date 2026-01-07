use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::Artifact;
use crate::utils::Id;

#[async_trait]
pub trait ArtifactRepository: Send + Sync {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Artifact>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Artifact>>;
    async fn insert(&self, artifact: &Artifact) -> anyhow::Result<()>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
}

pub struct PgArtifactRepository {
    pool: PgPool,
}

impl PgArtifactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArtifactRepository for PgArtifactRepository {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Artifact>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, name, uri, kind, created_at, updated_at
            FROM artifacts
            WHERE task_id = $1
            ORDER BY created_at DESC
            "#,
            task_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch artifacts")?;

        Ok(rows
            .into_iter()
            .map(|r| Artifact {
                id: r.id,
                task_id: r.task_id,
                name: r.name,
                uri: r.uri,
                kind: r.kind,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Artifact>> {
        todo!()
    }

    async fn insert(&self, artifact: &Artifact) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        todo!()
    }
}

