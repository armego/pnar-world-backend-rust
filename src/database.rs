use crate::{config::DatabaseSettings, error::AppResult};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{info, warn, debug, error};

pub async fn create_connection_pool(settings: &DatabaseSettings) -> AppResult<PgPool> {
    info!("Connecting to database at {}:{}", settings.host, settings.port);
    debug!("Database: {}, User: {}", settings.database_name, settings.username);

    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .min_connections(settings.min_connections)
        .acquire_timeout(settings.connect_timeout())
        .idle_timeout(Some(settings.idle_timeout()))
        .max_lifetime(Some(settings.max_lifetime()))
        .test_before_acquire(true)
        .connect_with(settings.connection_options())
        .await
        .map_err(|e| {
            error!("Failed to create database connection pool: {}", e);
            e
        })?;

    // Test the connection immediately
    info!("Testing database connection...");
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            error!("Database connection test failed: {}", e);
            e
        })?;

    info!(
        "Database connection pool created successfully with {}-{} connections", 
        settings.min_connections, 
        settings.max_connections
    );
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    info!("Running database migrations...");
    
    // Check if migrations table exists
    let migration_table_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = '_sqlx_migrations'
        )"
    )
    .fetch_one(pool)
    .await?;
    
    if !migration_table_exists.0 {
        info!("Migrations table does not exist, will be created");
    }
    
    // Run migrations with timeout
    let migration_result = tokio::time::timeout(
        Duration::from_secs(300), // 5 minute timeout for migrations
        sqlx::migrate!("./migrations").run(pool)
    ).await;
    
    match migration_result {
        Ok(Ok(_)) => {
            info!("Database migrations completed successfully");
            
            // Log applied migrations
            let applied_migrations: Vec<(String,)> = sqlx::query_as(
                "SELECT version FROM _sqlx_migrations ORDER BY version"
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default();
            
            if !applied_migrations.is_empty() {
                debug!("Applied migrations: {:?}", applied_migrations.iter().map(|(v,)| v).collect::<Vec<_>>());
            }
            
            Ok(())
        }
        Ok(Err(e)) => {
            error!("Migration failed: {}", e);
            Err(e.into())
        }
        Err(_) => {
            error!("Migration timed out after 5 minutes");
            Err(crate::error::AppError::Internal("Migration timeout".to_string()))
        }
    }
}

pub async fn health_check(pool: &PgPool) -> AppResult<DatabaseHealth> {
    let start = std::time::Instant::now();
    
    // Test basic connectivity
    sqlx::query("SELECT 1 as health_check")
        .fetch_one(pool)
        .await?;
    
    let response_time = start.elapsed();
    
    // Get pool statistics
    let pool_stats = get_pool_stats(pool).await;
    
    // Check database version
    let db_version: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| ("Unknown".to_string(),));
    
    let health = DatabaseHealth {
        status: "healthy".to_string(),
        response_time_ms: response_time.as_millis() as u64,
        pool_stats,
        database_version: db_version.0,
    };
    
    if response_time > Duration::from_millis(1000) {
        warn!("Database health check took {}ms (slow)", response_time.as_millis());
    } else {
        debug!("Database health check passed in {}ms", response_time.as_millis());
    }
    
    Ok(health)
}

pub async fn get_pool_stats(pool: &PgPool) -> PoolStats {
    let size = pool.size();
    let idle = pool.num_idle() as u32;
    let used = size.saturating_sub(idle);
    
    PoolStats {
        size,
        idle,
        used,
    }
}

/// Validates database connectivity and basic operations
pub async fn validate_connection(pool: &PgPool) -> AppResult<()> {
    info!("Validating database connection...");
    
    // Test connection
    health_check(pool).await?;
    
    // Test we can read from a system table
    let table_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_one(pool)
    .await?;
    
    debug!("Found {} public tables in database", table_count.0);
    
    info!("Database connection validation passed");
    Ok(())
}

/// Check if database is ready for application startup
pub async fn check_database_readiness(pool: &PgPool) -> AppResult<()> {
    info!("Checking database readiness...");
    
    // Validate connection
    validate_connection(pool).await?;
    
    // Check critical tables exist
    let critical_tables = vec![
        "users",
        "user_role", 
        "pnar_dictionary",
        "translation_requests",
    ];
    
    for table in critical_tables {
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_schema = 'public' 
                AND table_name = $1
            )"
        )
        .bind(table)
        .fetch_one(pool)
        .await?;
        
        if !exists.0 {
            error!("Critical table '{}' does not exist", table);
            return Err(crate::error::AppError::Internal(
                format!("Critical table '{}' missing", table)
            ));
        }
    }
    
    info!("Database readiness check passed");
    Ok(())
}

/// Perform database maintenance tasks
pub async fn perform_maintenance(pool: &PgPool) -> AppResult<MaintenanceReport> {
    info!("Performing database maintenance...");
    
    let start = std::time::Instant::now();
    
    // Analyze tables for better query performance
    sqlx::query("ANALYZE")
        .execute(pool)
        .await?;
    
    // Get database size
    let db_size: (String,) = sqlx::query_as(
        "SELECT pg_size_pretty(pg_database_size(current_database()))"
    )
    .fetch_one(pool)
    .await
    .unwrap_or_else(|_| ("Unknown".to_string(),));
    
    // Get table statistics
    let table_stats: Vec<(String, i64)> = sqlx::query_as(
        "SELECT schemaname||'.'||tablename as table_name, n_tup_ins + n_tup_upd + n_tup_del as total_operations
         FROM pg_stat_user_tables 
         ORDER BY total_operations DESC 
         LIMIT 10"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    
    let duration = start.elapsed();
    
    let report = MaintenanceReport {
        duration_ms: duration.as_millis() as u64,
        database_size: db_size.0,
        top_tables: table_stats,
        timestamp: chrono::Utc::now(),
    };
    
    info!("Database maintenance completed in {}ms", duration.as_millis());
    Ok(report)
}

#[derive(Debug, serde::Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub response_time_ms: u64,
    pub pool_stats: PoolStats,
    pub database_version: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PoolStats {
    pub size: u32,
    pub idle: u32,
    pub used: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct MaintenanceReport {
    pub duration_ms: u64,
    pub database_size: String,
    pub top_tables: Vec<(String, i64)>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}