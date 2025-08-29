use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// Standard API response wrapper for single items
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            pagination: None,
            timestamp: Utc::now(),
        }
    }
    
    pub fn with_pagination(data: T, pagination: PaginationInfo) -> Self {
        Self {
            data,
            pagination: Some(pagination),
            timestamp: Utc::now(),
        }
    }
}

/// Success message response
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl SuccessResponse {
    pub fn new(message: String) -> Self {
        Self {
            data: message,
            pagination: None,
            timestamp: Utc::now(),
        }
    }
}

/// User response (excluding sensitive data)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub translation_points: i32,
    pub bio: Option<String>,
    pub preferred_language: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Authentication response with tokens
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// API response for authentication operations
#[derive(Debug, Serialize)]
pub struct AuthApiResponse {
    pub data: AuthResponse,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl AuthApiResponse {
    pub fn new(data: AuthResponse) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}

/// API response for user operations
#[derive(Debug, Serialize)]
pub struct UserApiResponse {
    pub data: UserResponse,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl UserApiResponse {
    pub fn new(data: UserResponse) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}

/// Dictionary entry response
#[derive(Debug, Serialize)]
pub struct DictionaryEntryResponse {
    pub id: Uuid,
    pub pnar_word: String,
    pub pnar_word_kbf: Option<String>,
    pub english_word: String,
    pub part_of_speech: Option<String>,
    pub definition: Option<String>,
    pub example_pnar: Option<String>,
    pub example_english: Option<String>,
    pub difficulty_level: Option<i32>,
    pub usage_frequency: Option<i32>,
    pub cultural_context: Option<String>,
    pub related_words: Option<String>,
    pub pronunciation: Option<String>,
    pub etymology: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub created_by_email: Option<String>,
    pub updated_by: Option<Uuid>,
    pub updated_by_email: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_by_email: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, per_page: i64, total: i64) -> Self {
        let pages = (total.saturating_add(per_page).saturating_sub(1)) / per_page; // Safe division

        Self {
            data,
            pagination: PaginationInfo {
                page,
                per_page,
                total,
                pages,
            },
            timestamp: Utc::now(),
        }
    }
}

/// Dictionary entries paginated response
#[derive(Debug, Serialize)]
pub struct DictionaryPaginatedResponse {
    pub data: Vec<DictionaryEntryResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Users paginated response
#[derive(Debug, Serialize)]
pub struct UserPaginatedResponse {
    pub data: Vec<UserResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Translations paginated response
#[derive(Debug, Serialize)]
pub struct TranslationPaginatedResponse {
    pub data: Vec<TranslationResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Contributions paginated response
#[derive(Debug, Serialize)]
pub struct ContributionPaginatedResponse {
    pub data: Vec<ContributionResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Analytics paginated response
#[derive(Debug, Serialize)]
pub struct AnalyticsPaginatedResponse {
    pub data: Vec<AnalyticsResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Book paginated response
#[derive(Debug, Serialize)]
pub struct BookPaginatedResponse {
    pub data: Vec<crate::dto::book::BookResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Notification paginated response
#[derive(Debug, Serialize)]
pub struct NotificationPaginatedResponse {
    pub data: Vec<crate::dto::notification::NotificationResponse>,
    pub pagination: PaginationInfo,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

// Macro to generate paginated response implementations
macro_rules! impl_paginated_response {
    ($response_type:ty, $data_type:ty) => {
        impl $response_type {
            pub fn new(data: Vec<$data_type>, page: i64, per_page: i64, total: i64) -> Self {
                let pages = (total.saturating_add(per_page).saturating_sub(1)) / per_page;

                Self {
                    data,
                    pagination: PaginationInfo {
                        page,
                        per_page,
                        total,
                        pages,
                    },
                    timestamp: Utc::now(),
                }
            }
        }
    };
}

impl_paginated_response!(DictionaryPaginatedResponse, DictionaryEntryResponse);
impl_paginated_response!(UserPaginatedResponse, UserResponse);
impl_paginated_response!(TranslationPaginatedResponse, TranslationResponse);
impl_paginated_response!(ContributionPaginatedResponse, ContributionResponse);
impl_paginated_response!(AnalyticsPaginatedResponse, AnalyticsResponse);
impl_paginated_response!(BookPaginatedResponse, crate::dto::book::BookResponse);
impl_paginated_response!(NotificationPaginatedResponse, crate::dto::notification::NotificationResponse);

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub database: String,
}

impl HealthResponse {
    pub fn healthy(version: &str) -> Self {
        Self {
            status: "healthy".to_string(),
            version: version.to_string(),
            timestamp: Utc::now(),
            database: "connected".to_string(),
        }
    }

    pub fn unhealthy(version: &str, database_status: &str) -> Self {
        Self {
            status: "unhealthy".to_string(),
            version: version.to_string(),
            timestamp: Utc::now(),
            database: database_status.to_string(),
        }
    }
}

/// Translation request response
#[derive(Debug, Serialize)]
pub struct TranslationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: Option<String>,
    pub source_text: String,
    pub source_language: String,
    pub target_language: String,
    pub translated_text: Option<String>,
    pub status: String,
    pub translation_type: String,
    pub confidence_score: Option<f64>,
    pub reviewed: bool,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_by_email: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User contribution response
#[derive(Debug, Serialize)]
pub struct ContributionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: Option<String>,
    pub contribution_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub points_awarded: i32,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_by_email: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Word usage analytics response
#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub id: Uuid,
    pub word_id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
    pub usage_type: String,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<String>,
    pub context_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
