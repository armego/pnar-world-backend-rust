use actix_web::{delete, get, post, put, web, HttpResponse, Result};
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
    path = "/api/v1/contributions",
    tag = "contributions",
    request_body = CreateContributionRequest,
    responses(
        (status = 201, description = "Contribution created successfully", body = ContributionResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
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

/// Get contribution by ID
#[utoipa::path(
    get,
    path = "/api/v1/contributions/{id}",
    tag = "contributions",
    params(
        ("id" = Uuid, Path, description = "Contribution ID")
    ),
    responses(
        (status = 200, description = "Contribution retrieved successfully", body = ContributionResponse),
        (status = 404, description = "Contribution not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/{id}")]
pub async fn get_contribution(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let contribution =
        contribution_service::get_contribution(pool.get_ref(), path.into_inner(), None)
            .await?;

    Ok(HttpResponse::Ok().json(contribution))
}

/// List contributions (user's own or all if admin)
#[utoipa::path(
    get,
    path = "/api/v1/contributions",
    tag = "contributions",
    params(ContributionQueryParams),
    responses(
        (status = 200, description = "Contributions retrieved successfully", body = ContributionPaginatedResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("")]
pub async fn list_contributions(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<ContributionQueryParams>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    // For public access, show all contributions
    let user_id = None;

    let contributions =
        contribution_service::list_contributions(pool.get_ref(), user_id, page, per_page).await?;

    Ok(HttpResponse::Ok().json(contributions))
}

/// Update a contribution
#[utoipa::path(
    put,
    path = "/api/v1/contributions/{id}",
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
        ("bearer_auth" = [])
    )
)]
#[put("/{id}")]
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
    path = "/api/v1/contributions/{id}",
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
        ("bearer_auth" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_contribution(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    contribution_service::delete_contribution(pool.get_ref(), path.into_inner(), user.user_id)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
