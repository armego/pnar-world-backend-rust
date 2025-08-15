use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Request to create a new note
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNoteRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    #[schema(example = "My Note Title")]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    #[schema(example = "This is the content of my note")]
    pub content: String,

    #[schema(example = "personal")]
    pub category: Option<String>,

    #[schema(example = "important")]
    pub tags: Option<Vec<String>>,

    #[schema(example = false)]
    pub is_public: Option<bool>,
}

/// Request to update an existing note
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateNoteRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    #[schema(example = "Updated Note Title")]
    pub title: Option<String>,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    #[schema(example = "Updated content")]
    pub content: Option<String>,

    #[schema(example = "work")]
    pub category: Option<String>,

    #[schema(example = "updated,modified")]
    pub tags: Option<Vec<String>>,

    #[schema(example = true)]
    pub is_public: Option<bool>,
}

/// Note response
#[derive(Debug, Serialize, ToSchema)]
pub struct NoteResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "My Note Title")]
    pub title: String,
    #[schema(example = "This is the content of my note")]
    pub content: String,
    #[schema(example = "personal")]
    pub category: Option<String>,
    #[schema(example = "tag1,tag2")]
    pub tags: Option<Vec<String>>,
    #[schema(example = false)]
    pub is_public: bool,
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Notes search request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SearchNotesRequest {
    #[validate(length(min = 1, message = "Search query cannot be empty"))]
    #[schema(example = "important")]
    pub query: String,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    #[schema(example = 10)]
    pub limit: Option<i64>,

    #[schema(example = "personal")]
    pub category: Option<String>,

    #[schema(example = "tag1")]
    pub tag: Option<String>,
}
