use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Request to create a new notification
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNotificationRequest {
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "info")]
    pub r#type: String,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "New Message")]
    pub title: String,
    
    #[validate(length(min = 1, max = 1000))]
    #[schema(example = "You have received a new message")]
    pub message: String,
    
    #[schema(example = "{}")]
    pub data: Option<serde_json::Value>,
    
    #[schema(example = "2024-12-31T23:59:59Z")]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request to update a notification
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateNotificationRequest {
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "warning")]
    pub r#type: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Updated Title")]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 1000))]
    #[schema(example = "Updated message content")]
    pub message: Option<String>,
    
    #[schema(example = "{}")]
    pub data: Option<serde_json::Value>,
    
    #[schema(example = "2024-12-31T23:59:59Z")]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request to mark notification as read
#[derive(Debug, Deserialize, ToSchema)]
pub struct MarkNotificationReadRequest {
    #[schema(example = true)]
    pub read: bool,
}

/// Notification response
#[derive(Debug, Serialize, ToSchema)]
pub struct NotificationResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub user_id: Uuid,
    
    #[schema(example = "info")]
    pub r#type: String,
    
    #[schema(example = "New Message")]
    pub title: String,
    
    #[schema(example = "You have received a new message")]
    pub message: String,
    
    pub data: serde_json::Value,
    
    pub read: bool,
    
    pub read_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    
    pub expires_at: Option<DateTime<Utc>>,
}

/// Notification query parameters
#[derive(Debug, Deserialize, utoipa::IntoParams, ToSchema)]
pub struct NotificationQueryParams {
    #[schema(example = 1)]
    pub page: Option<i64>,
    
    #[schema(example = 20)]
    pub per_page: Option<i64>,
    
    #[schema(example = "info")]
    pub r#type: Option<String>,
    
    #[schema(example = false)]
    pub read: Option<bool>,
    
    #[schema(example = false)]
    pub include_expired: Option<bool>,
}
