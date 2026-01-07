use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::Artifact;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::ArtifactRepository;
use crate::utils::Id;

#[async_trait]
pub trait ArtifactService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Artifact>>;
    async fn create(&self, artifact: Artifact) -> AppResult<Artifact>;
    async fn delete(&self, task_id: Id, artifact_id: Id) -> AppResult<()>;
}

pub struct ArtifactServiceImpl {
    artifact_repo: Arc<dyn ArtifactRepository>,
}

impl ArtifactServiceImpl {
    pub fn new(artifact_repo: Arc<dyn ArtifactRepository>) -> Self {
        Self { artifact_repo }
    }
}

#[async_trait]
impl ArtifactService for ArtifactServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Artifact>> {
        self.artifact_repo
            .find_by_task(task_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn create(&self, artifact: Artifact) -> AppResult<Artifact> {
        self.artifact_repo
            .insert(&artifact)
            .await
            .map_err(AppError::Internal)?;

        Ok(artifact)
    }

    async fn delete(&self, task_id: Id, artifact_id: Id) -> AppResult<()> {
        let artifact = self.artifact_repo
            .find_by_id(artifact_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Artifact {} not found", artifact_id)))?;

        if artifact.task_id != task_id {
            return Err(AppError::BadRequest("Artifact does not belong to this task".into()));
        }

        let deleted = self.artifact_repo
            .delete(artifact_id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!("Artifact {} not found", artifact_id)));
        }

        Ok(())
    }
}
