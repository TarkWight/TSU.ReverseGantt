use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::Membership;
use crate::utils::Id;

#[async_trait]
pub trait MembershipRepository: Send + Sync {
    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Membership>>;
    async fn find_by_user(&self, user_id: Id) -> anyhow::Result<Vec<Membership>>;
    async fn find_by_project_and_user(&self, project_id: Id, user_id: Id) -> anyhow::Result<Option<Membership>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Membership>>;
    async fn find_leader(&self, project_id: Id) -> anyhow::Result<Option<Membership>>;
    async fn insert(&self, membership: &Membership) -> anyhow::Result<()>;
    async fn update_leader(&self, id: Id, is_leader: bool) -> anyhow::Result<bool>;
    async fn update_tags(&self, id: Id, tags: &[String]) -> anyhow::Result<bool>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
    async fn clear_leader(&self, project_id: Id) -> anyhow::Result<()>;
}

pub struct PgMembershipRepository {
    pool: PgPool,
}

impl PgMembershipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_tags(tags: Option<Vec<String>>) -> Vec<String> {
        tags.unwrap_or_default()
    }
}

#[async_trait]
impl MembershipRepository for PgMembershipRepository {
    async fn find_by_project(&self, project_id: Id) -> anyhow::Result<Vec<Membership>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, user_id, is_leader, tags
            FROM memberships WHERE project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch memberships")?;

        Ok(rows
            .into_iter()
            .map(|r| Membership {
                id: r.id,
                project_id: r.project_id,
                user_id: r.user_id,
                is_leader: r.is_leader,
                tags: Self::parse_tags(r.tags),
            })
            .collect())
    }

    async fn find_by_user(&self, user_id: Id) -> anyhow::Result<Vec<Membership>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, user_id, is_leader, tags
            FROM memberships WHERE user_id = $1
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch user memberships")?;

        Ok(rows
            .into_iter()
            .map(|r| Membership {
                id: r.id,
                project_id: r.project_id,
                user_id: r.user_id,
                is_leader: r.is_leader,
                tags: Self::parse_tags(r.tags),
            })
            .collect())
    }

    async fn find_by_project_and_user(&self, project_id: Id, user_id: Id) -> anyhow::Result<Option<Membership>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id, user_id, is_leader, tags
            FROM memberships WHERE project_id = $1 AND user_id = $2
            "#,
            project_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch membership")?;

        Ok(row.map(|r| Membership {
            id: r.id,
            project_id: r.project_id,
            user_id: r.user_id,
            is_leader: r.is_leader,
            tags: Self::parse_tags(r.tags),
        }))
    }

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Membership>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id, user_id, is_leader, tags
            FROM memberships WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch membership")?;

        Ok(row.map(|r| Membership {
            id: r.id,
            project_id: r.project_id,
            user_id: r.user_id,
            is_leader: r.is_leader,
            tags: Self::parse_tags(r.tags),
        }))
    }

    async fn find_leader(&self, project_id: Id) -> anyhow::Result<Option<Membership>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id, user_id, is_leader, tags
            FROM memberships WHERE project_id = $1 AND is_leader = true
            "#,
            project_id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch leader")?;

        Ok(row.map(|r| Membership {
            id: r.id,
            project_id: r.project_id,
            user_id: r.user_id,
            is_leader: r.is_leader,
            tags: Self::parse_tags(r.tags),
        }))
    }

    async fn insert(&self, membership: &Membership) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO memberships (id, project_id, user_id, is_leader, tags)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            membership.id,
            membership.project_id,
            membership.user_id,
            membership.is_leader,
            &membership.tags
        )
            .execute(&self.pool)
            .await
            .context("Failed to insert membership")?;

        Ok(())
    }

    async fn update_leader(&self, id: Id, is_leader: bool) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "UPDATE memberships SET is_leader = $2 WHERE id = $1",
            id,
            is_leader
        )
            .execute(&self.pool)
            .await
            .context("Failed to update leader")?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_tags(&self, id: Id, tags: &[String]) -> anyhow::Result<bool> {
        todo!()
    }

    async fn delete(&self, id: Id) -> anyhow::Result<bool> {
        todo!()
    }

    async fn clear_leader(&self, project_id: Id) -> anyhow::Result<()> {
        todo!()
    }
}
