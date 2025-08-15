use crate::{
    dto::{
        dictionary::{
            CreateDictionaryEntryRequest, SearchDictionaryRequest, UpdateDictionaryEntryRequest,
        },
        responses::ApiResponse,
    },
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::dictionary_service,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Create a new dictionary entry
#[utoipa::path(
    post,
    path = "/api/v1/dictionary",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    request_body = CreateDictionaryEntryRequest,
    responses(
        (status = 201, description = "Dictionary entry created successfully", body = ApiResponse<DictionaryEntryResponse>),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Dictionary entry already exists"),
        (status = 422, description = "Validation error")
    )
)]
#[post("")]
pub async fn create_entry(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    request: web::Json<CreateDictionaryEntryRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let entry = dictionary_service::create_entry(&pool, user.user_id, request.into_inner()).await?;

    Ok(HttpResponse::Created().json(ApiResponse::new(entry)))
}

/// Get a dictionary entry by ID
#[utoipa::path(
    get,
    path = "/api/v1/dictionary/{id}",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Dictionary entry ID")
    ),
    responses(
        (status = 200, description = "Dictionary entry retrieved successfully", body = ApiResponse<DictionaryEntryResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Dictionary entry not found")
    )
)]
#[get("/{id}")]
pub async fn get_entry(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    let entry = dictionary_service::get_entry(&pool, entry_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}

/// List dictionary entries with pagination
#[utoipa::path(
    get,
    path = "/api/v1/dictionary",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 20, max: 100)")
    ),
    responses(
        (status = 200, description = "Dictionary entries retrieved successfully", body = DictionaryPaginatedResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
#[get("")]
pub async fn list_entries(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationQuery>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let result = dictionary_service::list_entries(&pool, page, per_page).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Search dictionary entries
#[utoipa::path(
    post,
    path = "/api/v1/dictionary/search",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    request_body = SearchDictionaryRequest,
    responses(
        (status = 200, description = "Search results retrieved successfully", body = ApiResponse<Vec<DictionaryEntryResponse>>),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Validation error")
    )
)]
#[post("/search")]
pub async fn search_entries(
    pool: web::Data<PgPool>,
    request: web::Json<SearchDictionaryRequest>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let entries = dictionary_service::search_entries(&pool, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entries)))
}

/// Update a dictionary entry
#[utoipa::path(
    put,
    path = "/api/v1/dictionary/{id}",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Dictionary entry ID")
    ),
    request_body = UpdateDictionaryEntryRequest,
    responses(
        (status = 200, description = "Dictionary entry updated successfully", body = ApiResponse<DictionaryEntryResponse>),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Dictionary entry not found"),
        (status = 422, description = "Validation error")
    )
)]
#[put("/{id}")]
pub async fn update_entry(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdateDictionaryEntryRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let entry_id = path.into_inner();
    let entry =
        dictionary_service::update_entry(&pool, entry_id, user.user_id, request.into_inner())
            .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}

/// Delete a dictionary entry
#[utoipa::path(
    delete,
    path = "/api/v1/dictionary/{id}",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Dictionary entry ID")
    ),
    responses(
        (status = 204, description = "Dictionary entry deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Dictionary entry not found")
    )
)]
#[delete("/{id}")]
pub async fn delete_entry(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    dictionary_service::delete_entry(&pool, entry_id, user.user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Verify a dictionary entry
#[utoipa::path(
    put,
    path = "/api/v1/dictionary/{id}/verify",
    tag = "dictionary",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Dictionary entry ID")
    ),
    responses(
        (status = 200, description = "Dictionary entry verified successfully", body = ApiResponse<DictionaryEntryResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Dictionary entry not found")
    )
)]
#[put("/{id}/verify")]
pub async fn verify_entry(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    let entry = dictionary_service::verify_entry(&pool, entry_id, user.user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}
