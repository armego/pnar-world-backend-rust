use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Request to create a new notification
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNotificationRequest {
    #[validate(length(min = 1, max = 50))]
    pub r#type: String,
    
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(min = 1, max = 1000))]
    pub message: String,
    
    pub data: Option<serde_json::Value>,
    
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request to update a notification
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNotificationRequest {
    #[validate(length(min = 1, max = 50))]
    pub r#type: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 1000))]
    pub message: Option<String>,
    
    pub data: Option<serde_json::Value>,
    
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request to mark notification as read
#[derive(Debug, Deserialize)]
pub struct MarkNotificationReadRequest {
    pub read: bool,
}

/// Notification response
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    
    pub user_id: Uuid,
    
    pub r#type: String,
    
    pub title: String,
    
    pub message: String,
    
    pub data: serde_json::Value,
    
    pub read: bool,
    
    pub read_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    
    pub expires_at: Option<DateTime<Utc>>,
}

/// Notification query parameters
#[derive(Debug, Deserialize)]
pub struct NotificationQueryParams {
    pub page: Option<i64>,
    
    pub per_page: Option<i64>,
    
    pub r#type: Option<String>,
    
    pub read: Option<bool>,
    
    pub include_expired: Option<bool>,
}
