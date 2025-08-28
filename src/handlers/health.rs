use crate::{database, error::AppError, state::AppState};
use actix_web::{get, web, HttpResponse};
use serde_json::json;
use std::time::Instant;

)]
#[get("/health")]
pub async fn health_check(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let start_time = Instant::now();
    let version = env!("CARGO_PKG_VERSION");
    let db = state.get_db_pool();

    match db {
        Some(pool) => {
            match database::health_check(&pool).await {
                Ok(db_health) => {
                    let total_time = start_time.elapsed();
                    
                    let health_data = json!({
                        "status": "healthy",
                        "version": version,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "uptime_seconds": get_uptime_seconds(),
                        "response_time_ms": total_time.as_millis(),
                        "database": {
                            "status": db_health.status,
                            "response_time_ms": db_health.response_time_ms,
                            "pool_stats": db_health.pool_stats,
                            "version": db_health.database_version
                        },
                        "system": get_system_info(),
                        "environment": std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
                    });

                    tracing::debug!("Health check passed - all systems operational");
                    Ok(HttpResponse::Ok().json(health_data))
                },
                Err(e) => {
                    let total_time = start_time.elapsed();
                    
                    let health_data = json!({
                        "status": "unhealthy",
                        "version": version,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "uptime_seconds": get_uptime_seconds(),
                        "response_time_ms": total_time.as_millis(),
                        "database": {
                            "status": "unhealthy",
                            "error": "Database connection failed"
                        },
                        "system": get_system_info(),
                        "environment": std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
                    });

                    tracing::warn!("Health check failed - database error: {}", e);
                    Ok(HttpResponse::ServiceUnavailable().json(health_data))
                }
            }
        },
        None => {
            let total_time = start_time.elapsed();
            
            let health_data = json!({
                "status": "unhealthy",
                "version": version,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "uptime_seconds": get_uptime_seconds(),
                "response_time_ms": total_time.as_millis(),
                "database": {
                    "status": "unavailable",
                    "error": "Database not initialized"
                },
                "system": get_system_info(),
                "environment": std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
            });

            tracing::error!("Health check failed - no database connection available");
            Ok(HttpResponse::ServiceUnavailable().json(health_data))
        }
    }
}

)]
#[get("/ready")]
pub async fn readiness_check(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db = state.get_db_pool();

    match db {
        Some(pool) => {
            // Check if database is ready for queries
            match database::check_database_readiness(&pool).await {
                Ok(_) => {
                    tracing::debug!("Readiness check passed");
                    Ok(HttpResponse::Ok().json(json!({
                        "status": "ready",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })))
                },
                Err(e) => {
                    tracing::warn!("Readiness check failed: {}", e);
                    Ok(HttpResponse::ServiceUnavailable().json(json!({
                        "status": "not_ready",
                        "error": "Database not ready",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })))
                }
            }
        },
        None => {
            tracing::error!("Readiness check failed - no database connection");
            Ok(HttpResponse::ServiceUnavailable().json(json!({
                "status": "not_ready",
                "error": "Database not available",
                "timestamp": chrono::Utc::now().to_rfc3339()
            })))
        }
    }
}

)]
#[get("/live")]
pub async fn liveness_check() -> Result<HttpResponse, AppError> {
    // Simple liveness check - just return OK if the service is running
    Ok(HttpResponse::Ok().json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}

)]
#[get("/metrics")]
pub async fn metrics(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let start_time = Instant::now();
    let db = state.get_db_pool();

    let mut metrics = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": get_uptime_seconds(),
        "system": get_system_info()
    });

    if let Some(pool) = db.as_ref() {
        let pool_stats = database::get_pool_stats(pool).await;
        metrics["database"] = json!({
            "pool_size": pool_stats.size,
            "pool_idle": pool_stats.idle,
            "pool_used": pool_stats.used,
            "pool_utilization": (pool_stats.used as f64 / pool_stats.size as f64) * 100.0
        });
    }

    let response_time = start_time.elapsed();
    metrics["response_time_ms"] = json!(response_time.as_millis());

    Ok(HttpResponse::Ok().json(metrics))
}

fn get_uptime_seconds() -> u64 {
    static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START_TIME.get_or_init(|| Instant::now());
    start.elapsed().as_secs()
}

fn get_system_info() -> serde_json::Value {
    json!({
        "rust_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        "target": std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
        "build_timestamp": std::env::var("BUILD_TIMESTAMP").unwrap_or_else(|_| "unknown".to_string()),
        "git_hash": option_env!("GIT_HASH").unwrap_or("unknown"),
        "memory_usage": get_memory_usage(),
        "cpu_count": num_cpus::get()
    })
}

fn get_memory_usage() -> serde_json::Value {
    // Basic memory usage information
    // In production, you might want to use a more sophisticated memory monitoring library
    json!({
        "rss_bytes": get_rss_memory().unwrap_or(0),
        "virtual_bytes": get_virtual_memory().unwrap_or(0)
    })
}

#[cfg(target_os = "linux")]
fn get_rss_memory() -> Option<u64> {
    use std::fs;
    let status = fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].parse::<u64>().ok().map(|kb| kb * 1024);
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn get_virtual_memory() -> Option<u64> {
    use std::fs;
    let status = fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if line.starts_with("VmSize:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].parse::<u64>().ok().map(|kb| kb * 1024);
            }
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn get_rss_memory() -> Option<u64> {
    None
}

#[cfg(not(target_os = "linux"))]
fn get_virtual_memory() -> Option<u64> {
    None
}
