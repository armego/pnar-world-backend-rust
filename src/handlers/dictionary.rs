use crate::{
    dto::{
        dictionary::{
            CreateDictionaryEntryRequest, SearchDictionaryRequest, UpdateDictionaryEntryRequest,
        },
        responses::ApiResponse,
    },
    error::AppError,
    middleware::{
        auth::ModeratorUser,
        hierarchy::ManagerUser,
    },
    services::dictionary_service,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Create a new dictionary entry
#[post("")]
pub async fn create_entry(
    pool: web::Data<PgPool>,
    user: ManagerUser, // Require admin privileges for dictionary creation
    request: web::Json<CreateDictionaryEntryRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let entry = dictionary_service::create_entry(&pool, user.0.user_id, request.into_inner()).await?;

    Ok(HttpResponse::Created().json(ApiResponse::new(entry)))
}

/// Get a dictionary entry by ID
#[get("/{id}")]
pub async fn get_entry(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    
    // Extract analytics data from request
    let session_id = None; // Could be extracted from headers/cookies
    let ip_address = req.peer_addr().map(|addr| addr.ip().to_string());
    let user_agent = req.headers().get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let entry = dictionary_service::get_entry(
        &pool, 
        entry_id, 
        None, // No user_id for anonymous access
        session_id,
        ip_address,
        user_agent,
    ).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}

/// List dictionary entries with pagination
#[get("")]
pub async fn list_entries(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let result = dictionary_service::list_entries(&pool, page, per_page).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Search dictionary entries
#[post("/search")]
pub async fn search_entries(
    pool: web::Data<PgPool>,
    request: web::Json<SearchDictionaryRequest>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    // Extract analytics data from request
    let session_id = None; // Could be extracted from headers/cookies
    let ip_address = req.peer_addr().map(|addr| addr.ip().to_string());
    let user_agent = req.headers().get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let entries = dictionary_service::search_entries(
        &pool, 
        request.into_inner(),
        None, // No user_id for anonymous access
        session_id,
        ip_address,
        user_agent,
    ).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entries)))
}

/// Update a dictionary entry
#[put("/{id}")]
pub async fn update_entry(
    pool: web::Data<PgPool>,
    user: ManagerUser, // Require admin privileges for dictionary updates
    path: web::Path<Uuid>,
    request: web::Json<UpdateDictionaryEntryRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let entry_id = path.into_inner();
    let entry =
        dictionary_service::update_entry(&pool, entry_id, user.0.user_id, request.into_inner())
            .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}

/// Delete a dictionary entry
#[delete("/{id}")]
pub async fn delete_entry(
    pool: web::Data<PgPool>,
    user: ManagerUser, // Require admin privileges for dictionary deletion
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    dictionary_service::delete_entry(&pool, entry_id, user.0.user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Verify a dictionary entry
#[put("/{id}/verify")]
pub async fn verify_entry(
    pool: web::Data<PgPool>,
    user: ModeratorUser, // Require moderator role or higher
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let entry_id = path.into_inner();
    let entry = dictionary_service::verify_entry(&pool, entry_id, user.0.user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(entry)))
}
