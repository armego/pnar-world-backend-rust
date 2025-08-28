use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Book response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub language: String,
    pub genre: Option<String>,
    pub page_count: Option<i32>,
    pub cover_image_url: Option<String>,
    pub pdf_url: Option<String>,
    pub epub_url: Option<String>,
    pub status: String,
    pub difficulty_level: Option<i32>,
    pub is_public: bool,
    pub tags: Option<Vec<String>>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create book request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,

    #[validate(length(min = 1, max = 255, message = "Author must be between 1 and 255 characters"))]
    pub author: String,

    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,

    #[validate(length(max = 20, message = "ISBN must be less than 20 characters"))]
    pub isbn: Option<String>,

    #[validate(length(max = 255, message = "Publisher must be less than 255 characters"))]
    pub publisher: Option<String>,

    pub publication_date: Option<NaiveDate>,

    #[validate(length(min = 2, max = 10, message = "Language must be between 2 and 10 characters"))]
    pub language: String,

    #[validate(length(max = 100, message = "Genre must be less than 100 characters"))]
    pub genre: Option<String>,

    #[validate(range(min = 1, message = "Page count must be at least 1"))]
    pub page_count: Option<i32>,

    #[validate(url(message = "Cover image URL must be valid"))]
    pub cover_image_url: Option<String>,

    #[validate(url(message = "PDF URL must be valid"))]
    pub pdf_url: Option<String>,

    #[validate(url(message = "EPUB URL must be valid"))]
    pub epub_url: Option<String>,

    #[validate(length(max = 50, message = "Status must be less than 50 characters"))]
    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    pub difficulty_level: Option<i32>,

    pub is_public: Option<bool>,

    pub tags: Option<Vec<String>>,
}

/// Update book request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 1, max = 255, message = "Author must be between 1 and 255 characters"))]
    pub author: Option<String>,

    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,

    #[validate(length(max = 20, message = "ISBN must be less than 20 characters"))]
    pub isbn: Option<String>,

    #[validate(length(max = 255, message = "Publisher must be less than 255 characters"))]
    pub publisher: Option<String>,

    pub publication_date: Option<NaiveDate>,

    #[validate(length(min = 2, max = 10, message = "Language must be between 2 and 10 characters"))]
    pub language: Option<String>,

    #[validate(length(max = 100, message = "Genre must be less than 100 characters"))]
    pub genre: Option<String>,

    #[validate(range(min = 1, message = "Page count must be at least 1"))]
    pub page_count: Option<i32>,

    #[validate(url(message = "Cover image URL must be valid"))]
    pub cover_image_url: Option<String>,

    #[validate(url(message = "PDF URL must be valid"))]
    pub pdf_url: Option<String>,

    #[validate(url(message = "EPUB URL must be valid"))]
    pub epub_url: Option<String>,

    #[validate(length(max = 50, message = "Status must be less than 50 characters"))]
    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    pub difficulty_level: Option<i32>,

    pub is_public: Option<bool>,

    pub tags: Option<Vec<String>>,
}

/// Book query parameters for listing books
#[derive(Debug, Deserialize, Validate)]
pub struct BookQueryParams {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<i64>,

    #[validate(range(min = 1, max = 100, message = "Per page must be between 1 and 100"))]
    pub per_page: Option<i64>,

    pub language: Option<String>,

    pub genre: Option<String>,

    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    pub difficulty_level: Option<i32>,

    pub is_public: Option<bool>,

    pub search: Option<String>, // Search in title, author, or description

    pub tag: Option<String>, // Filter by specific tag
}
