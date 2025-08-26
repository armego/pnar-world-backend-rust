use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

/// Book response structure
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    #[schema(example = "Ka Jingiathuh Pnar")]
    pub title: String,

    #[validate(length(min = 1, max = 255, message = "Author must be between 1 and 255 characters"))]
    #[schema(example = "U Kong Pakyntein")]
    pub author: String,

    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    #[schema(example = "Traditional Pnar folktales and stories")]
    pub description: Option<String>,

    #[validate(length(max = 20, message = "ISBN must be less than 20 characters"))]
    #[schema(example = "978-1234567890")]
    pub isbn: Option<String>,

    #[validate(length(max = 255, message = "Publisher must be less than 255 characters"))]
    #[schema(example = "Pnar Publications")]
    pub publisher: Option<String>,

    #[schema(example = "2023-12-01")]
    pub publication_date: Option<NaiveDate>,

    #[validate(length(min = 2, max = 10, message = "Language must be between 2 and 10 characters"))]
    #[schema(example = "pnar")]
    pub language: String,

    #[validate(length(max = 100, message = "Genre must be less than 100 characters"))]
    #[schema(example = "folklore")]
    pub genre: Option<String>,

    #[validate(range(min = 1, message = "Page count must be at least 1"))]
    #[schema(example = 150)]
    pub page_count: Option<i32>,

    #[validate(url(message = "Cover image URL must be valid"))]
    #[schema(example = "https://example.com/cover.jpg")]
    pub cover_image_url: Option<String>,

    #[validate(url(message = "PDF URL must be valid"))]
    #[schema(example = "https://example.com/book.pdf")]
    pub pdf_url: Option<String>,

    #[validate(url(message = "EPUB URL must be valid"))]
    #[schema(example = "https://example.com/book.epub")]
    pub epub_url: Option<String>,

    #[validate(length(max = 50, message = "Status must be less than 50 characters"))]
    #[schema(example = "draft")]
    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    #[schema(example = 2)]
    pub difficulty_level: Option<i32>,

    #[schema(example = false)]
    pub is_public: Option<bool>,

    #[schema(example = json!(["folklore", "traditional"]))]
    pub tags: Option<Vec<String>>,
}

/// Update book request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    #[schema(example = "Ka Jingiathuh Pnar - Updated")]
    pub title: Option<String>,

    #[validate(length(min = 1, max = 255, message = "Author must be between 1 and 255 characters"))]
    #[schema(example = "U Kong Pakyntein")]
    pub author: Option<String>,

    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    #[schema(example = "Updated description")]
    pub description: Option<String>,

    #[validate(length(max = 20, message = "ISBN must be less than 20 characters"))]
    #[schema(example = "978-1234567890")]
    pub isbn: Option<String>,

    #[validate(length(max = 255, message = "Publisher must be less than 255 characters"))]
    #[schema(example = "Pnar Publications")]
    pub publisher: Option<String>,

    #[schema(example = "2023-12-01")]
    pub publication_date: Option<NaiveDate>,

    #[validate(length(min = 2, max = 10, message = "Language must be between 2 and 10 characters"))]
    #[schema(example = "pnar")]
    pub language: Option<String>,

    #[validate(length(max = 100, message = "Genre must be less than 100 characters"))]
    #[schema(example = "folklore")]
    pub genre: Option<String>,

    #[validate(range(min = 1, message = "Page count must be at least 1"))]
    #[schema(example = 200)]
    pub page_count: Option<i32>,

    #[validate(url(message = "Cover image URL must be valid"))]
    #[schema(example = "https://example.com/cover.jpg")]
    pub cover_image_url: Option<String>,

    #[validate(url(message = "PDF URL must be valid"))]
    #[schema(example = "https://example.com/book.pdf")]
    pub pdf_url: Option<String>,

    #[validate(url(message = "EPUB URL must be valid"))]
    #[schema(example = "https://example.com/book.epub")]
    pub epub_url: Option<String>,

    #[validate(length(max = 50, message = "Status must be less than 50 characters"))]
    #[schema(example = "published")]
    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    #[schema(example = 3)]
    pub difficulty_level: Option<i32>,

    #[schema(example = true)]
    pub is_public: Option<bool>,

    #[schema(example = json!(["folklore", "traditional", "updated"]))]
    pub tags: Option<Vec<String>>,
}

/// Book query parameters for listing books
#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct BookQueryParams {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    #[schema(example = 1)]
    pub page: Option<i64>,

    #[validate(range(min = 1, max = 100, message = "Per page must be between 1 and 100"))]
    #[schema(example = 10)]
    pub per_page: Option<i64>,

    #[schema(example = "pnar")]
    pub language: Option<String>,

    #[schema(example = "folklore")]
    pub genre: Option<String>,

    #[schema(example = "published")]
    pub status: Option<String>,

    #[validate(range(min = 1, max = 5, message = "Difficulty level must be between 1 and 5"))]
    #[schema(example = 2)]
    pub difficulty_level: Option<i32>,

    #[schema(example = true)]
    pub is_public: Option<bool>,

    #[schema(example = "traditional")]
    pub search: Option<String>, // Search in title, author, or description

    #[schema(example = "folklore")]
    pub tag: Option<String>, // Filter by specific tag
}
