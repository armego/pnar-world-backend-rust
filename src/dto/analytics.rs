use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

/// Request to record word usage analytics
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAnalyticsRequest {
    pub word_id: Uuid,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Event type must be between 1 and 50 characters"
    ))]
    pub event_type: String,

    pub timestamp: DateTime<Utc>,

    #[validate(length(max = 255, message = "Session ID must be less than 255 characters"))]
    pub session_id: Option<String>,

    pub metadata: Option<serde_json::Value>,
}

/// Request to update analytics record
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAnalyticsRequest {
    pub metadata: Option<serde_json::Value>,
}
