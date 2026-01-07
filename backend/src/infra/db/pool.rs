use sqlx::{PgPool, migrate::Migrator};
use std::path::Path;

pub async fn create_pool(database_url: &str) -> Result<PgPool, anyhow::Error> {
    tracing::info!("Connecting to database...");
    let pool = PgPool::connect(database_url).await?;
    tracing::info!("Database connection established");

    run_migrations(&pool).await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), anyhow::Error> {
    tracing::info!("Running database migrations...");

    let migrations_path = Path::new("./migrations");

    let migrations_path = if migrations_path.exists() {
        migrations_path
    } else {
        Path::new("./backend/migrations")
    };

    if !migrations_path.exists() {
        return Err(anyhow::anyhow!(
            "Migrations directory not found. Tried: {} and ./backend/migrations",
            migrations_path.display()
        ));
    }

    tracing::info!("Loading migrations from: {}", migrations_path.display());

    let migrator = Migrator::new(migrations_path).await
        .map_err(|e| {
            tracing::error!("Failed to load migrations: {}", e);
            anyhow::anyhow!("Failed to load migrations: {}", e)
        })?;

    migrator
        .run(pool)
        .await
        .map_err(|e| {
            tracing::error!("Migration error: {}", e);
            anyhow::anyhow!("Failed to run migrations: {}", e)
        })?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}

