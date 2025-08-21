use crate::{database, dto::HealthResponse, error::AppError, state::AppState};
use actix_web::{get, web, HttpResponse};
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
pub async fn health_check(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let version = env!("CARGO_PKG_VERSION");
    let db = state.db.read().await;

    match db.as_ref() {
        Some(pool) => match database::health_check(pool).await {
            Ok(_) => Ok(HttpResponse::Ok().json(HealthResponse::healthy(version))),
            Err(_) => Ok(HttpResponse::ServiceUnavailable()
                .json(HealthResponse::unhealthy(version, "database error"))),
        },
        None => Ok(HttpResponse::ServiceUnavailable()
            .json(HealthResponse::unhealthy(version, "database not connected"))),
    }
}
