use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Request to record word usage analytics
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAnalyticsRequest {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub word_id: Uuid,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Event type must be between 1 and 50 characters"
    ))]
    #[schema(example = "search")]
    pub event_type: String,

    #[schema(example = "2023-01-01T00:00:00Z")]
    pub timestamp: DateTime<Utc>,

    #[validate(length(max = 255, message = "Session ID must be less than 255 characters"))]
    #[schema(example = "sess_12345")]
    pub session_id: Option<String>,

    pub metadata: Option<serde_json::Value>,
}

/// Request to update analytics record
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAnalyticsRequest {
    pub metadata: Option<serde_json::Value>,
}
