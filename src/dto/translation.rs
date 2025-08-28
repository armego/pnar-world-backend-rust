use serde::Deserialize;
use validator::Validate;

/// Request to create a new translation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTranslationRequest {
    #[validate(length(
        min = 1,
        max = 5000,
        message = "Source text must be between 1 and 5000 characters"
    ))]
    pub source_text: String,

    #[validate(length(
        min = 2,
        max = 10,
        message = "Source language must be between 2 and 10 characters"
    ))]
    pub source_language: Option<String>,

    #[validate(length(
        min = 2,
        max = 10,
        message = "Target language must be between 2 and 10 characters"
    ))]
    pub target_language: Option<String>,

    pub translation_type: Option<String>,

    pub metadata: Option<serde_json::Value>,
}

/// Request to update a translation request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTranslationRequest {
    #[validate(length(
        min = 1,
        max = 5000,
        message = "Translated text must be between 1 and 5000 characters"
    ))]
    pub translated_text: Option<String>,

    #[validate(length(max = 50, message = "Status must be less than 50 characters"))]
    pub status: Option<String>,

    #[validate(range(
        min = 0.0,
        max = 1.0,
        message = "Confidence score must be between 0 and 1"
    ))]
    pub confidence_score: Option<f64>,

    pub reviewed: Option<bool>,

    pub metadata: Option<serde_json::Value>,
}
