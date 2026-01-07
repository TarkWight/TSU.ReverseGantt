use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Context;

use crate::domain::{User, GlobalRole};
use crate::utils::Id;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<(User, String)>>;
    async fn find_all(&self) -> anyhow::Result<Vec<User>>;
    async fn insert(&self, user: &User, password_hash: &str) -> anyhow::Result<()>;
    async fn update_global_role(&self, id: Id, role: GlobalRole) -> anyhow::Result<bool>;
    async fn update_email_notifications(&self, id: Id, enabled: bool) -> anyhow::Result<bool>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn parse_global_role(role: &str) -> GlobalRole {
        role
            .parse()
            .unwrap_or(GlobalRole::Student)
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<User>> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<(User, String)>> {
        todo!()
    }

    async fn find_all(&self) -> anyhow::Result<Vec<User>> {
        todo!()
    }

    async fn insert(&self, user: &User, password_hash: &str) -> anyhow::Result<()> {
        todo!()
    }

    async fn update_global_role(&self, id: Id, role: GlobalRole) -> anyhow::Result<bool> {
        todo!()
    }

    async fn update_email_notifications(&self, id: Id, enabled: bool) -> anyhow::Result<bool> {
        todo!()
    }
}

