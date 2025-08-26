use crate::{
    dto::{
        book::{BookQueryParams, CreateBookRequest, UpdateBookRequest},
        responses::{ApiResponse, SuccessResponse},
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
#[utoipa::path(
    post,
    path = "/api/v1/books",
    tag = "books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created successfully", body = crate::dto::book::BookResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Contributor access required"),
        (status = 409, description = "Book with this ISBN already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
pub async fn create_book(
    pool: web::Data<PgPool>,
    request: web::Json<CreateBookRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if !authorization::has_minimum_role_level(&auth_user.role, "admin") {
        return Err(AppError::Forbidden(
            "Book creation requires admin privileges",
        ));
    }

    request.validate()?;
    let book = book_service::create_book(&pool, request.into_inner(), auth_user.user_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse::new(book)))
}

/// Get a book by ID
#[utoipa::path(
    get,
    path = "/api/v1/books/{id}",
    tag = "books",
    params(
        ("id" = Uuid, Path, description = "Book ID")
    ),
    responses(
        (status = 200, description = "Book retrieved successfully", body = crate::dto::book::BookResponse),
        (status = 404, description = "Book not found")
    )
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
            ));
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::new(book)))
}

/// List books with pagination and filtering
#[utoipa::path(
    get,
    path = "/api/v1/books",
    tag = "books",
    params(BookQueryParams),
    responses(
        (status = 200, description = "Books retrieved successfully", body = crate::dto::responses::BookPaginatedResponse),
        (status = 400, description = "Invalid query parameters")
    )
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
#[utoipa::path(
    put,
    path = "/api/v1/books/{id}",
    tag = "books",
    params(
        ("id" = Uuid, Path, description = "Book ID")
    ),
    request_body = UpdateBookRequest,
    responses(
        (status = 200, description = "Book updated successfully", body = crate::dto::book::BookResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Insufficient permissions"),
        (status = 404, description = "Book not found"),
        (status = 409, description = "Book with this ISBN already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
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
        ));
    }

    let updated_book = book_service::update_book(&pool, book_id, request.into_inner(), auth_user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(updated_book)))
}

/// Delete a book
#[utoipa::path(
    delete,
    path = "/api/v1/books/{id}",
    tag = "books",
    params(
        ("id" = Uuid, Path, description = "Book ID")
    ),
    responses(
        (status = 200, description = "Book deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Insufficient permissions"),
        (status = 404, description = "Book not found")
    ),
    security(
        ("bearer_auth" = [])
    )
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
        ));
    }

    book_service::delete_book(&pool, book_id).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::new("Book deleted successfully".to_string())))
}

/// Get books by current user
#[utoipa::path(
    get,
    path = "/api/v1/books/mine",
    tag = "books",
    params(BookQueryParams),
    responses(
        (status = 200, description = "User's books retrieved successfully", body = crate::dto::responses::BookPaginatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid query parameters")
    ),
    security(
        ("bearer_auth" = [])
    )
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
    );

    Ok(HttpResponse::Ok().json(filtered_response))
}
