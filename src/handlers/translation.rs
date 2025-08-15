use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    dto::{CreateTranslationRequest, UpdateTranslationRequest},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::translation_service,
};

#[derive(Deserialize, IntoParams)]
pub struct TranslationQueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Create a new translation request
#[utoipa::path(
    post,
    path = "/api/translations",
    tag = "translations",
    request_body = CreateTranslationRequest,
    responses(
        (status = 201, description = "Translation request created successfully", body = TranslationResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_translation(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    req: web::Json<CreateTranslationRequest>,
) -> Result<HttpResponse, AppError> {
    let translation = translation_service::create_translation_request(
        pool.get_ref(),
        user.user_id,
        req.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Created().json(translation))
}

/// Get a translation request by ID
#[utoipa::path(
    get,
    path = "/api/translations/{id}",
    tag = "translations",
    params(
        ("id" = Uuid, Path, description = "Translation request ID")
    ),
    responses(
        (status = 200, description = "Translation request retrieved successfully", body = TranslationResponse),
        (status = 404, description = "Translation request not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_translation(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let translation = translation_service::get_translation_request(
        pool.get_ref(),
        path.into_inner(),
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(translation))
}

/// List translation requests for the authenticated user
#[utoipa::path(
    get,
    path = "/api/translations",
    tag = "translations",
    params(TranslationQueryParams),
    responses(
        (status = 200, description = "Translation requests retrieved successfully", body = Vec<TranslationResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn list_translations(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    query: web::Query<TranslationQueryParams>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let translations = translation_service::list_translation_requests(
        pool.get_ref(),
        user.user_id,
        page,
        per_page,
    )
    .await?;

    Ok(HttpResponse::Ok().json(translations))
}

/// Update a translation request
#[utoipa::path(
    put,
    path = "/api/translations/{id}",
    tag = "translations",
    params(
        ("id" = Uuid, Path, description = "Translation request ID")
    ),
    request_body = UpdateTranslationRequest,
    responses(
        (status = 200, description = "Translation request updated successfully", body = TranslationResponse),
        (status = 404, description = "Translation request not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_translation(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    req: web::Json<UpdateTranslationRequest>,
) -> Result<HttpResponse, AppError> {
    let translation = translation_service::update_translation_request(
        pool.get_ref(),
        path.into_inner(),
        user.user_id,
        req.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(translation))
}

/// Delete a translation request
#[utoipa::path(
    delete,
    path = "/api/translations/{id}",
    tag = "translations",
    params(
        ("id" = Uuid, Path, description = "Translation request ID")
    ),
    responses(
        (status = 204, description = "Translation request deleted successfully"),
        (status = 404, description = "Translation request not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_translation(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    translation_service::delete_translation_request(
        pool.get_ref(),
        path.into_inner(),
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}
