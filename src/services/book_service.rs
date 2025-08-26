use crate::{
    constants::{defaults, error_messages},
    dto::{book::*, responses::PaginatedResponse},
    error::{AppError, AppResult},
};
use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Create a new book
pub async fn create_book(
    pool: &PgPool,
    request: CreateBookRequest,
    created_by: Uuid,
) -> AppResult<BookResponse> {
    // Check if book with same ISBN already exists (if ISBN provided)
    if let Some(ref isbn) = request.isbn {
        let existing_book = sqlx::query("SELECT id FROM books WHERE isbn = $1")
            .bind(isbn)
            .fetch_optional(pool)
            .await?;

        if existing_book.is_some() {
            return Err(AppError::Conflict("Book with this ISBN already exists"));
        }
    }

    let book_id = Uuid::new_v4();
    let now = Utc::now();

    let book_row = sqlx::query(
        r#"
        INSERT INTO books (
            id, title, author, description, isbn, publisher, publication_date,
            language, genre, page_count, cover_image_url, pdf_url, epub_url,
            status, difficulty_level, is_public, tags, created_by, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
        RETURNING 
            id, title, author, description, isbn, publisher, publication_date,
            language, genre, page_count, cover_image_url, pdf_url, epub_url,
            status, difficulty_level, is_public, tags, created_by, updated_by,
            created_at, updated_at
        "#,
    )
    .bind(book_id)
    .bind(&request.title)
    .bind(&request.author)
    .bind(&request.description)
    .bind(&request.isbn)
    .bind(&request.publisher)
    .bind(&request.publication_date)
    .bind(&request.language)
    .bind(&request.genre)
    .bind(&request.page_count)
    .bind(&request.cover_image_url)
    .bind(&request.pdf_url)
    .bind(&request.epub_url)
    .bind(&request.status.as_deref().unwrap_or("draft"))
    .bind(&request.difficulty_level)
    .bind(&request.is_public.unwrap_or(false))
    .bind(&request.tags)
    .bind(created_by)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(row_to_book_response(book_row))
}

/// Get a book by ID
pub async fn get_book_by_id(pool: &PgPool, book_id: Uuid) -> AppResult<BookResponse> {
    let book_row = sqlx::query(
        r#"
        SELECT 
            id, title, author, description, isbn, publisher, publication_date,
            language, genre, page_count, cover_image_url, pdf_url, epub_url,
            status, difficulty_level, is_public, tags, created_by, updated_by,
            created_at, updated_at
        FROM books 
        WHERE id = $1
        "#,
    )
    .bind(book_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(error_messages::BOOK_NOT_FOUND))?;

    Ok(row_to_book_response(book_row))
}

/// List books with pagination and filtering
pub async fn list_books(
    pool: &PgPool,
    params: BookQueryParams,
    include_private: bool,
) -> AppResult<PaginatedResponse<BookResponse>> {
    let page = params.page.unwrap_or(defaults::PAGE);
    let per_page = params.per_page.unwrap_or(defaults::PER_PAGE);
    let offset = (page - 1) * per_page;

    // Build dynamic WHERE clause
    let mut where_conditions = Vec::new();
    let mut bind_count = 0;

    // Public visibility filter
    if !include_private {
        bind_count += 1;
        where_conditions.push(format!("is_public = ${}", bind_count));
    }

    // Language filter
    if params.language.is_some() {
        bind_count += 1;
        where_conditions.push(format!("language = ${}", bind_count));
    }

    // Genre filter
    if params.genre.is_some() {
        bind_count += 1;
        where_conditions.push(format!("genre = ${}", bind_count));
    }

    // Status filter
    if params.status.is_some() {
        bind_count += 1;
        where_conditions.push(format!("status = ${}", bind_count));
    }

    // Difficulty level filter
    if params.difficulty_level.is_some() {
        bind_count += 1;
        where_conditions.push(format!("difficulty_level = ${}", bind_count));
    }

    // Search filter
    if params.search.is_some() {
        bind_count += 1;
        where_conditions.push(format!(
            "(title ILIKE ${} OR author ILIKE ${} OR description ILIKE ${})",
            bind_count, bind_count, bind_count
        ));
    }

    // Tag filter
    if params.tag.is_some() {
        bind_count += 1;
        where_conditions.push(format!("${} = ANY(tags)", bind_count));
    }

    let where_clause = if where_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as count FROM books {}",
        where_clause
    );

    // Data query
    let data_query = format!(
        r#"
        SELECT 
            id, title, author, description, isbn, publisher, publication_date,
            language, genre, page_count, cover_image_url, pdf_url, epub_url,
            status, difficulty_level, is_public, tags, created_by, updated_by,
            created_at, updated_at
        FROM books 
        {} 
        ORDER BY created_at DESC 
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        bind_count + 1,
        bind_count + 2
    );

    // Build and execute count query
    let mut count_query_builder = sqlx::query(&count_query);
    let mut data_query_builder = sqlx::query(&data_query);

    // Pre-compute search pattern to avoid lifetime issues
    let search_pattern = params.search.as_ref().map(|search| format!("%{}%", search));

    // Bind parameters in order
    let mut current_bind = 1;

    if !include_private {
        count_query_builder = count_query_builder.bind(true);
        data_query_builder = data_query_builder.bind(true);
        current_bind += 1;
    }

    if let Some(ref language) = params.language {
        count_query_builder = count_query_builder.bind(language);
        data_query_builder = data_query_builder.bind(language);
        current_bind += 1;
    }

    if let Some(ref genre) = params.genre {
        count_query_builder = count_query_builder.bind(genre);
        data_query_builder = data_query_builder.bind(genre);
        current_bind += 1;
    }

    if let Some(ref status) = params.status {
        count_query_builder = count_query_builder.bind(status);
        data_query_builder = data_query_builder.bind(status);
        current_bind += 1;
    }

    if let Some(difficulty_level) = params.difficulty_level {
        count_query_builder = count_query_builder.bind(difficulty_level);
        data_query_builder = data_query_builder.bind(difficulty_level);
        current_bind += 1;
    }

    if let Some(ref pattern) = search_pattern {
        count_query_builder = count_query_builder.bind(pattern);
        data_query_builder = data_query_builder.bind(pattern);
        current_bind += 1;
    }

    if let Some(ref tag) = params.tag {
        count_query_builder = count_query_builder.bind(tag);
        data_query_builder = data_query_builder.bind(tag);
        current_bind += 1;
    }

    // Add limit and offset
    data_query_builder = data_query_builder.bind(per_page).bind(offset);

    // Execute queries
    let count_result = count_query_builder.fetch_one(pool).await?;
    let total_count: i64 = count_result.get("count");

    let book_rows = data_query_builder.fetch_all(pool).await?;

    let books: Vec<BookResponse> = book_rows
        .into_iter()
        .map(row_to_book_response)
        .collect();

    Ok(PaginatedResponse::new(books, page, per_page, total_count))
}

/// Update a book
pub async fn update_book(
    pool: &PgPool,
    book_id: Uuid,
    request: UpdateBookRequest,
    updated_by: Uuid,
) -> AppResult<BookResponse> {
    // Check if book exists
    let existing_book = get_book_by_id(pool, book_id).await?;

    // Check ISBN uniqueness if it's being updated
    if let Some(ref isbn) = request.isbn {
        if existing_book.isbn.as_ref() != Some(isbn) {
            let existing_isbn = sqlx::query("SELECT id FROM books WHERE isbn = $1 AND id != $2")
                .bind(isbn)
                .bind(book_id)
                .fetch_optional(pool)
                .await?;

            if existing_isbn.is_some() {
                return Err(AppError::Conflict("Book with this ISBN already exists"));
            }
        }
    }

    let book_row = sqlx::query(
        r#"
        UPDATE books 
        SET 
            title = COALESCE($2, title),
            author = COALESCE($3, author),
            description = COALESCE($4, description),
            isbn = COALESCE($5, isbn),
            publisher = COALESCE($6, publisher),
            publication_date = COALESCE($7, publication_date),
            language = COALESCE($8, language),
            genre = COALESCE($9, genre),
            page_count = COALESCE($10, page_count),
            cover_image_url = COALESCE($11, cover_image_url),
            pdf_url = COALESCE($12, pdf_url),
            epub_url = COALESCE($13, epub_url),
            status = COALESCE($14, status),
            difficulty_level = COALESCE($15, difficulty_level),
            is_public = COALESCE($16, is_public),
            tags = COALESCE($17, tags),
            updated_by = $18,
            updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, title, author, description, isbn, publisher, publication_date,
            language, genre, page_count, cover_image_url, pdf_url, epub_url,
            status, difficulty_level, is_public, tags, created_by, updated_by,
            created_at, updated_at
        "#,
    )
    .bind(book_id)
    .bind(&request.title)
    .bind(&request.author)
    .bind(&request.description)
    .bind(&request.isbn)
    .bind(&request.publisher)
    .bind(&request.publication_date)
    .bind(&request.language)
    .bind(&request.genre)
    .bind(&request.page_count)
    .bind(&request.cover_image_url)
    .bind(&request.pdf_url)
    .bind(&request.epub_url)
    .bind(&request.status)
    .bind(&request.difficulty_level)
    .bind(&request.is_public)
    .bind(&request.tags)
    .bind(updated_by)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(error_messages::BOOK_NOT_FOUND))?;

    Ok(row_to_book_response(book_row))
}

/// Delete a book
pub async fn delete_book(pool: &PgPool, book_id: Uuid) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(book_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(error_messages::BOOK_NOT_FOUND));
    }

    Ok(())
}

/// Helper function to convert database row to BookResponse
fn row_to_book_response(row: sqlx::postgres::PgRow) -> BookResponse {
    BookResponse {
        id: row.get("id"),
        title: row.get("title"),
        author: row.get("author"),
        description: row.get("description"),
        isbn: row.get("isbn"),
        publisher: row.get("publisher"),
        publication_date: row.get("publication_date"),
        language: row.get("language"),
        genre: row.get("genre"),
        page_count: row.get("page_count"),
        cover_image_url: row.get("cover_image_url"),
        pdf_url: row.get("pdf_url"),
        epub_url: row.get("epub_url"),
        status: row.get("status"),
        difficulty_level: row.get("difficulty_level"),
        is_public: row.get("is_public"),
        tags: row.get("tags"),
        created_by: row.get("created_by"),
        updated_by: row.get("updated_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
