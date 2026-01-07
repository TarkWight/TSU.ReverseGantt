use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{Assignment, AssignRole};
use crate::utils::Id;

#[async_trait]
pub trait AssignmentRepository: Send + Sync {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Assignment>>;
    async fn find_by_user(&self, user_id: Id) -> anyhow::Result<Vec<Assignment>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Assignment>>;
    async fn find_by_task_and_user(&self, task_id: Id, user_id: Id) -> anyhow::Result<Option<Assignment>>;
    async fn find_owner(&self, task_id: Id) -> anyhow::Result<Option<Assignment>>;
    async fn insert(&self, assignment: &Assignment) -> anyhow::Result<()>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
    async fn delete_by_task_and_user(&self, task_id: Id, user_id: Id) -> anyhow::Result<bool>;
    async fn exists(&self, task_id: Id, user_id: Id, role: AssignRole) -> anyhow::Result<bool>;
}

pub struct PgAssignmentRepository {
    pool: PgPool,
}

impl PgAssignmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_role(role: &str) -> AssignRole {
        role.parse().unwrap_or(AssignRole::Assignee)
    }
}

#[async_trait]
impl AssignmentRepository for PgAssignmentRepository {
    async fn find_by_task(&self, task_id: Id) -> anyhow::Result<Vec<Assignment>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, user_id, role
            FROM assignments WHERE task_id = $1
            ORDER BY role, user_id
            "#,
            task_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch assignments")?;

        Ok(rows
            .into_iter()
            .map(|r| Assignment {
                id: r.id,
                task_id: r.task_id,
                user_id: r.user_id,
                role: Self::parse_role(&r.role),
            })
            .collect())
    }

    async fn find_by_user(&self, user_id: Id) -> anyhow::Result<Vec<Assignment>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, user_id, role
            FROM assignments WHERE user_id = $1
            ORDER BY role, task_id
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch user assignments")?;

        Ok(rows
            .into_iter()
            .map(|r| Assignment {
                id: r.id,
                task_id: r.task_id,
                user_id: r.user_id,
                role: Self::parse_role(&r.role),
            })
            .collect())
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Assignment>> {
        let row = sqlx::query!(
            r#"
            SELECT id, task_id, user_id, role
            FROM assignments WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch assignment")?;

        Ok(row.map(|r| Assignment {
            id: r.id,
            task_id: r.task_id,
            user_id: r.user_id,
            role: Self::parse_role(&r.role),
        }))
    }

    async fn find_by_task_and_user(&self, task_id: Id, user_id: Id) -> anyhow::Result<Option<Assignment>> {
        todo!()
    }

    async fn find_owner(&self, task_id: Id) -> anyhow::Result<Option<Assignment>> {
        todo!()
    }

    async fn insert(&self, assignment: &Assignment) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn delete_by_task_and_user(&self, task_id: Id, user_id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn exists(&self, task_id: Id, user_id: Id, role: AssignRole) -> anyhow::Result<bool> {
        todo!()
    }
}
