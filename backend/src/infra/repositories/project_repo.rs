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
    
}