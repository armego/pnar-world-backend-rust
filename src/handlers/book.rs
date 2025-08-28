use crate::{
    dto::{
        book::{BookQueryParams, CreateBookRequest, UpdateBookRequest},
    },
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::book_service,
    utils::authorization,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// Create a new book
#[post("")]
pub async fn create_book(
    pool: web::Data<PgPool>,
    request: web::Json<CreateBookRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if !authorization::has_minimum_role_level(&auth_user.role, "admin") {
        return Err(AppError::Forbidden(
            "Book creation requires admin privileges",
    }

    request.validate()?;
    let book = book_service::create_book(&pool, request.into_inner(), auth_user.user_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse::new(book)))
}

/// Get a book by ID
    params(
)]
#[get("/{id}")]
pub async fn get_book(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    auth_user: Option<AuthenticatedUser>,
) -> Result<HttpResponse, AppError> {
    let book_id = path.into_inner();
    let book = book_service::get_book_by_id(&pool, book_id).await?;

    if !book.is_public {
        let auth_user = auth_user.ok_or_else(|| {
            AppError::Unauthorized("Authentication required for private books")
        })?;

        if book.created_by != auth_user.user_id && !authorization::has_minimum_role_level(&auth_user.role, "admin") {
            return Err(AppError::Forbidden(
                "You don't have permission to view this private book",
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::new(book)))
}

/// List books with pagination and filtering
    params(BookQueryParams),
)]
#[get("")]
pub async fn list_books(
    pool: web::Data<PgPool>,
    query: web::Query<BookQueryParams>,
    auth_user: Option<AuthenticatedUser>,
) -> Result<HttpResponse, AppError> {
    query.validate()?;

    let include_private = auth_user
        .as_ref()
        .map(|user| authorization::has_minimum_role_level(&user.role, "admin"))
        .unwrap_or(false);

    let books = book_service::list_books(&pool, query.into_inner(), include_private).await?;
    Ok(HttpResponse::Ok().json(books))
}

/// Update a book
    params(
)]
#[put("/{id}")]
pub async fn update_book(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateBookRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let book_id = path.into_inner();
    request.validate()?;

    let existing_book = book_service::get_book_by_id(&pool, book_id).await?;

    if existing_book.created_by != auth_user.user_id && !authorization::has_minimum_role_level(&auth_user.role, "admin") {
        return Err(AppError::Forbidden(
            "You can only update your own books or need admin privileges",
    }

    let updated_book = book_service::update_book(&pool, book_id, request.into_inner(), auth_user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(updated_book)))
}

/// Delete a book
    params(
)]
#[delete("/{id}")]
pub async fn delete_book(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let book_id = path.into_inner();
    let existing_book = book_service::get_book_by_id(&pool, book_id).await?;

    if existing_book.created_by != auth_user.user_id && !authorization::has_minimum_role_level(&auth_user.role, "admin") {
        return Err(AppError::Forbidden(
            "You can only delete your own books or need admin privileges",
    }

    book_service::delete_book(&pool, book_id).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::new("Book deleted successfully".to_string())))
}

/// Get books by current user
    params(BookQueryParams),
)]
#[get("/mine")]
pub async fn get_my_books(
    pool: web::Data<PgPool>,
    query: web::Query<BookQueryParams>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    query.validate()?;

    let books = book_service::list_books(&pool, query.into_inner(), true).await?;

    let user_books = books.data.into_iter()
        .filter(|book| book.created_by == auth_user.user_id)
        .collect();

    let filtered_response = crate::dto::responses::PaginatedResponse::new(
        user_books,
        books.pagination.page,
        books.pagination.per_page,
        books.pagination.total,

    Ok(HttpResponse::Ok().json(filtered_response))
}
