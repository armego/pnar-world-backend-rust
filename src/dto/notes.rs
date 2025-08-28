use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Request to create a new note
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    pub category: Option<String>,

    pub tags: Option<Vec<String>>,

    pub is_public: Option<bool>,
}

/// Request to update an existing note
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: Option<String>,

    pub category: Option<String>,

    pub tags: Option<Vec<String>>,

    pub is_public: Option<bool>,
}

/// Note response
#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Notes search request
#[derive(Debug, Deserialize, Validate)]
pub struct SearchNotesRequest {
    #[validate(length(min = 1, message = "Search query cannot be empty"))]
    pub query: String,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<i64>,

    pub category: Option<String>,

    pub tag: Option<String>,
}
