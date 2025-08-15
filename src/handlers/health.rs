use crate::{database, dto::HealthResponse, error::AppError};
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use utoipa;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy", body = HealthResponse)
    )
)]
#[get("/health")]
pub async fn health_check(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let version = env!("CARGO_PKG_VERSION");
    
    match database::health_check(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(HealthResponse::healthy(version))),
        Err(_) => Ok(HttpResponse::ServiceUnavailable()
            .json(HealthResponse::unhealthy(version, "disconnected"))),
    }
}
