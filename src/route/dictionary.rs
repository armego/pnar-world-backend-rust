use crate::middleware::auth::AuthDetails;
use crate::model::request::dictionary::{CreateDictionaryRequest, UpdateDictionaryRequest, SearchDictionaryRequest};
use crate::model::response::ErrorResponse;
use crate::server::AppState;
use crate::service::dictionary::{
    create_dictionary_entry, get_dictionary_entry, get_all_dictionary_entries,
    search_dictionary_entries, update_dictionary_entry, delete_dictionary_entry,
    verify_dictionary_entry,
};
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put, HttpResponse};
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[doc = "API Resource: /dictionary [POST] - Create a new dictionary entry"]
#[tracing::instrument(
    name = "Creating dictionary entry",
    skip(body, auth, data),
    fields(
        user_id = %auth.id,
        pnar_word = body.pnar_word
    )
)]
#[post("/dictionary")]
pub async fn create_dictionary_handler(
    body: Json<CreateDictionaryRequest>,
    auth: AuthDetails,
    data: Data<AppState>,
) -> HttpResponse {
    match create_dictionary_entry(body.into_inner(), auth.id, &data.db).await {
        Ok(entry) => HttpResponse::Created().json(json!(entry)),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary/{id} [GET] - Get dictionary entry by ID"]
#[tracing::instrument(
    name = "Getting dictionary entry",
    skip(data),
    fields(
        entry_id = %path.as_ref()
    )
)]
#[get("/dictionary/{id}")]
pub async fn get_dictionary_handler(
    path: Path<uuid::Uuid>,
    data: Data<AppState>,
) -> HttpResponse {
    let id = path.into_inner();
    match get_dictionary_entry(id, &data.db).await {
        Ok(Some(entry)) => HttpResponse::Ok().json(json!(entry)),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Dictionary entry not found"})),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary [GET] - Get all dictionary entries with pagination"]
#[tracing::instrument(
    name = "Getting all dictionary entries",
    skip(data),
    fields(
        limit = query.limit,
        offset = query.offset
    )
)]
#[get("/dictionary")]
pub async fn get_all_dictionary_handler(
    query: Query<PaginationQuery>,
    data: Data<AppState>,
) -> HttpResponse {
    match get_all_dictionary_entries(query.limit, query.offset, &data.db).await {
        Ok(entries) => HttpResponse::Ok().json(json!(entries)),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary/search [POST] - Search dictionary entries"]
#[tracing::instrument(
    name = "Searching dictionary entries",
    skip(body, data),
    fields(
        query = body.query,
        search_type = body.search_type
    )
)]
#[post("/dictionary/search")]
pub async fn search_dictionary_handler(
    body: Json<SearchDictionaryRequest>,
    data: Data<AppState>,
) -> HttpResponse {
    match search_dictionary_entries(body.into_inner(), &data.db).await {
        Ok(entries) => HttpResponse::Ok().json(json!(entries)),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary/{id} [PUT] - Update dictionary entry"]
#[tracing::instrument(
    name = "Updating dictionary entry",
    skip(body, auth, data),
    fields(
        user_id = %auth.id,
        entry_id = %path.as_ref()
    )
)]
#[put("/dictionary/{id}")]
pub async fn update_dictionary_handler(
    path: Path<uuid::Uuid>,
    body: Json<UpdateDictionaryRequest>,
    auth: AuthDetails,
    data: Data<AppState>,
) -> HttpResponse {
    let id = path.into_inner();
    match update_dictionary_entry(id, body.into_inner(), auth.id, &data.db).await {
        Ok(entry) => HttpResponse::Ok().json(json!(entry)),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary/{id} [DELETE] - Delete dictionary entry"]
#[tracing::instrument(
    name = "Deleting dictionary entry",
    skip(auth, data),
    fields(
        user_id = %auth.id,
        entry_id = %path.as_ref()
    )
)]
#[delete("/dictionary/{id}")]
pub async fn delete_dictionary_handler(
    path: Path<uuid::Uuid>,
    auth: AuthDetails,
    data: Data<AppState>,
) -> HttpResponse {
    let id = path.into_inner();
    match delete_dictionary_entry(id, &data.db).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(json!({"error": "Dictionary entry not found"})),
        Err(code) => ErrorResponse::build(code),
    }
}

#[doc = "API Resource: /dictionary/{id}/verify [PUT] - Verify dictionary entry"]
#[tracing::instrument(
    name = "Verifying dictionary entry",
    skip(auth, data),
    fields(
        user_id = %auth.id,
        entry_id = %path.as_ref()
    )
)]
#[put("/dictionary/{id}/verify")]
pub async fn verify_dictionary_handler(
    path: Path<uuid::Uuid>,
    auth: AuthDetails,
    data: Data<AppState>,
) -> HttpResponse {
    let id = path.into_inner();
    match verify_dictionary_entry(id, auth.id, &data.db).await {
        Ok(entry) => HttpResponse::Ok().json(json!(entry)),
        Err(code) => ErrorResponse::build(code),
    }
}
