#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_all(&self) -> anyhow::Result<Vec<Project>>;
    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Project>>;
    async fn insert(&self, project: &Project) -> anyhow::Result<()>;
    async fn update(&self, project: &Project) -> anyhow::Result<bool>;
    async fn delete(&self, id: Id) -> anyhow::Result<bool>;
}

pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl ProjectRepository for ProjectRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Project>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, start_date, due_date, created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch projects")?;

        Ok(rows
            .into_iter()
            .map(|row| Project {
                id: row.id,
                name: row.name,
                description: row.description,
                start_date: row.start_date,
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect())
    }
}