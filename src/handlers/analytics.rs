use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    dto::{CreateAnalyticsRequest, UpdateAnalyticsRequest},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::analytics_service,
};

#[derive(Deserialize, IntoParams)]
pub struct AnalyticsQueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Uuid>,
    pub word_id: Option<Uuid>,
    pub event_type: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct WordStatsParams {
    pub user_id: Option<Uuid>,
}

/// Create a new analytics record
#[utoipa::path(
    post,
    path = "/api/analytics",
    tag = "analytics",
    request_body = CreateAnalyticsRequest,
    responses(
        (status = 201, description = "Analytics record created successfully", body = AnalyticsResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_analytics(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    req: web::Json<CreateAnalyticsRequest>,
) -> Result<HttpResponse, AppError> {
    let analytics = analytics_service::create_analytics_record(
        pool.get_ref(),
        Some(user.user_id),
        req.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Created().json(analytics))
}

/// Create an anonymous analytics record (no authentication required)
#[utoipa::path(
    post,
    path = "/api/analytics/anonymous",
    tag = "analytics",
    request_body = CreateAnalyticsRequest,
    responses(
        (status = 201, description = "Anonymous analytics record created successfully", body = AnalyticsResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_anonymous_analytics(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<CreateAnalyticsRequest>,
) -> Result<HttpResponse, AppError> {
    let analytics =
        analytics_service::create_analytics_record(pool.get_ref(), None, req.into_inner()).await?;

    Ok(HttpResponse::Created().json(analytics))
}

/// Get an analytics record by ID
#[utoipa::path(
    get,
    path = "/api/analytics/{id}",
    tag = "analytics",
    params(
        ("id" = Uuid, Path, description = "Analytics record ID")
    ),
    responses(
        (status = 200, description = "Analytics record retrieved successfully", body = AnalyticsResponse),
        (status = 404, description = "Analytics record not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_analytics(
    pool: web::Data<sqlx::PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let analytics =
        analytics_service::get_analytics_record(pool.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(analytics))
}

/// List analytics records with filtering
#[utoipa::path(
    get,
    path = "/api/analytics",
    tag = "analytics",
    params(AnalyticsQueryParams),
    responses(
        (status = 200, description = "Analytics records retrieved successfully", body = AnalyticsPaginatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn list_analytics(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    query: web::Query<AnalyticsQueryParams>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    // Only allow viewing all user analytics if user is admin
    let user_id = if user.role == "admin" {
        query.user_id
    } else {
        Some(user.user_id)
    };

    let analytics = analytics_service::list_analytics_records(
        pool.get_ref(),
        user_id,
        query.word_id,
        query.event_type.clone(),
        page,
        per_page,
    )
    .await?;

    Ok(HttpResponse::Ok().json(analytics))
}

/// Update an analytics record
#[utoipa::path(
    put,
    path = "/api/analytics/{id}",
    tag = "analytics",
    params(
        ("id" = Uuid, Path, description = "Analytics record ID")
    ),
    request_body = UpdateAnalyticsRequest,
    responses(
        (status = 200, description = "Analytics record updated successfully", body = AnalyticsResponse),
        (status = 404, description = "Analytics record not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_analytics(
    pool: web::Data<sqlx::PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    req: web::Json<UpdateAnalyticsRequest>,
) -> Result<HttpResponse, AppError> {
    let analytics = analytics_service::update_analytics_record(
        pool.get_ref(),
        path.into_inner(),
        req.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(analytics))
}

/// Delete an analytics record
#[utoipa::path(
    delete,
    path = "/api/analytics/{id}",
    tag = "analytics",
    params(
        ("id" = Uuid, Path, description = "Analytics record ID")
    ),
    responses(
        (status = 204, description = "Analytics record deleted successfully"),
        (status = 404, description = "Analytics record not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_analytics(
    pool: web::Data<sqlx::PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    analytics_service::delete_analytics_record(pool.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get word usage statistics
#[utoipa::path(
    get,
    path = "/api/analytics/words/{word_id}/stats",
    tag = "analytics",
    params(
        ("word_id" = Uuid, Path, description = "Word ID"),
        WordStatsParams
    ),
    responses(
        (status = 200, description = "Word usage statistics retrieved successfully", body = serde_json::Value),
        (status = 404, description = "Word not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_word_stats(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<WordStatsParams>,
) -> Result<HttpResponse, AppError> {
    // Only allow viewing all user stats if user is admin
    let user_id = if user.role == "admin" {
        query.user_id
    } else {
        Some(user.user_id)
    };

    let stats =
        analytics_service::get_word_usage_stats(pool.get_ref(), path.into_inner(), user_id).await?;

    Ok(HttpResponse::Ok().json(stats))
}
