use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    dto::{CreateContributionRequest, UpdateContributionRequest},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::contribution_service,
};

#[derive(Deserialize, IntoParams)]
pub struct ContributionQueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub all: Option<bool>,
}

/// Create a new contribution
#[utoipa::path(
    post,
    path = "/api/contributions",
    tag = "contributions",
    request_body = CreateContributionRequest,
    responses(
        (status = 201, description = "Contribution created successfully", body = ContributionResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_contribution(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    req: web::Json<CreateContributionRequest>,
) -> Result<HttpResponse, AppError> {
    let contribution =
        contribution_service::create_contribution(pool.get_ref(), user.user_id, req.into_inner())
            .await?;

    Ok(HttpResponse::Created().json(contribution))
}

/// Get a contribution by ID
#[utoipa::path(
    get,
    path = "/api/contributions/{id}",
    tag = "contributions",
    params(
        ("id" = Uuid, Path, description = "Contribution ID")
    ),
    responses(
        (status = 200, description = "Contribution retrieved successfully", body = ContributionResponse),
        (status = 404, description = "Contribution not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_contribution(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let contribution =
        contribution_service::get_contribution(pool.get_ref(), path.into_inner(), user.user_id)
            .await?;

    Ok(HttpResponse::Ok().json(contribution))
}

/// List contributions (user's own or all if admin)
#[utoipa::path(
    get,
    path = "/api/contributions",
    tag = "contributions",
    params(ContributionQueryParams),
    responses(
        (status = 200, description = "Contributions retrieved successfully", body = Vec<ContributionResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn list_contributions(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    query: web::Query<ContributionQueryParams>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    // Only allow viewing all contributions if user is admin
    let user_id = if query.all.unwrap_or(false) && user.role == "admin" {
        None
    } else {
        Some(user.user_id)
    };

    let contributions =
        contribution_service::list_contributions(pool.get_ref(), user_id, page, per_page).await?;

    Ok(HttpResponse::Ok().json(contributions))
}

/// Update a contribution
#[utoipa::path(
    put,
    path = "/api/contributions/{id}",
    tag = "contributions",
    params(
        ("id" = Uuid, Path, description = "Contribution ID")
    ),
    request_body = UpdateContributionRequest,
    responses(
        (status = 200, description = "Contribution updated successfully", body = ContributionResponse),
        (status = 404, description = "Contribution not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_contribution(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    req: web::Json<UpdateContributionRequest>,
) -> Result<HttpResponse, AppError> {
    let contribution = contribution_service::update_contribution(
        pool.get_ref(),
        path.into_inner(),
        user.user_id,
        req.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(contribution))
}

/// Delete a contribution
#[utoipa::path(
    delete,
    path = "/api/contributions/{id}",
    tag = "contributions",
    params(
        ("id" = Uuid, Path, description = "Contribution ID")
    ),
    responses(
        (status = 204, description = "Contribution deleted successfully"),
        (status = 404, description = "Contribution not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_contribution(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    contribution_service::delete_contribution(pool.get_ref(), path.into_inner(), user.user_id)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
