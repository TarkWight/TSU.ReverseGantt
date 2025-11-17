use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, anyhow::Error> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}