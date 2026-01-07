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

    async fn find_by_id(&self, id: Id) -> anyhow::Result<Option<Project>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, start_date, due_date, created_at, updated_at
            FROM projects
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch project")?;

        Ok(row.map(|r| Project {
            id: r.id,
            name: r.name,
            description: r.description,
            start_date: r.start_date,
            due_date: r.due_date,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn insert(&self, project: &Project) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, description, start_date, due_date, created_at, updated_at)
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
            .context("Failed to insert project")?;

        Ok(())
    }

    async fn update(&self, project: &Project) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE projects
            SET name = $2, description = $3, start_date = $4, due_date = $5, updated_at = $6
            WHERE id = $1
            "#,
            project.id,
            project.name,
            project.description,
            project.start_date,
            project.due_date,
            project.updated_at
        )
            .execute(&self.pool)
            .await
            .context("Failed to update project")?;

        Ok(result.rows_affected() > 0)
    }
}