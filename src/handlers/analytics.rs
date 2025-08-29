use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::{
    dto::{CreateAnalyticsRequest, UpdateAnalyticsRequest},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::analytics_service,
};

#[derive(serde::Deserialize)]
pub struct AnalyticsQueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Uuid>,
    pub word_id: Option<Uuid>,
    pub usage_type: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct WordStatsParams {
    pub user_id: Option<Uuid>,
}

/// Create a new analytics record
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
pub async fn create_anonymous_analytics(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<CreateAnalyticsRequest>,
) -> Result<HttpResponse, AppError> {
    let analytics =
        analytics_service::create_analytics_record(pool.get_ref(), None, req.into_inner()).await?;

    Ok(HttpResponse::Created().json(analytics))
}

/// Get an analytics record by ID
pub async fn get_analytics(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let analytics =
        analytics_service::get_analytics_record(pool.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(analytics))
}

/// List analytics records with filtering
pub async fn list_analytics(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<AnalyticsQueryParams>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    // For public access, allow viewing analytics without user restriction
    let user_id = query.user_id;

    let analytics = analytics_service::list_analytics_records(
        pool.get_ref(),
        user_id,
        query.word_id,
        query.usage_type.as_deref(),
        page,
        per_page,
    )
    .await?;

    Ok(HttpResponse::Ok().json(analytics))
}

/// Update an analytics record
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
pub async fn delete_analytics(
    pool: web::Data<sqlx::PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    analytics_service::delete_analytics_record(pool.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get word usage statistics
pub async fn get_word_stats(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    query: web::Query<WordStatsParams>,
) -> Result<HttpResponse, AppError> {
    // For public access, allow viewing stats without user restriction
    let user_id = query.user_id;

    let stats =
        analytics_service::get_word_usage_stats(pool.get_ref(), path.into_inner(), user_id).await?;

    Ok(HttpResponse::Ok().json(stats))
}
