use crate::{config::DatabaseSettings, error::AppResult};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::info;

pub async fn create_connection_pool(settings: &DatabaseSettings) -> AppResult<PgPool> {
    info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(settings.connection_options())
        .await?;

    info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Database migrations completed successfully");
    Ok(())
}

pub async fn health_check(pool: &PgPool) -> AppResult<()> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
