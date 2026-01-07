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
        todo!()
    }

    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Dependency>> {
        todo!()
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Dependency>> {
        todo!()
    }

    async fn insert(&self, dependency: &Dependency) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn get_task_project_id(&self, task_id: Id) -> anyhow::Result<Option<Id>> {
        todo!()
    }

    async fn get_project_task_ids(&self, project_id: Id) -> anyhow::Result<Vec<Id>> {
        todo!()
    }
}

