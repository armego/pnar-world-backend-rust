use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// Standard API response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}

/// Success message response
#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    pub data: String,
    pub timestamp: DateTime<Utc>,
}

impl SuccessResponse {
    pub fn new(message: String) -> Self {
        Self {
            data: message,
            timestamp: Utc::now(),
        }
    }
}

/// User response (excluding sensitive data)
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub full_name: Option<String>,
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,
    #[schema(example = "user")]
    pub role: String,
    #[schema(example = 100)]
    pub translation_points: i32,
    #[schema(example = "Language enthusiast")]
    pub bio: Option<String>,
    #[schema(example = "en")]
    pub preferred_language: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Authentication response with tokens
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub user: UserResponse,
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub access_token: String,
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub refresh_token: String,
    #[schema(example = 86400)]
    pub expires_in: i64,
}

/// API response for authentication operations
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthApiResponse {
    pub data: AuthResponse,
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
#[derive(Debug, Serialize, ToSchema)]
pub struct UserApiResponse {
    pub data: UserResponse,
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
#[derive(Debug, Serialize, ToSchema)]
pub struct DictionaryEntryResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "ka")]
    pub pnar_word: String,
    #[schema(example = "go")]
    pub english_word: String,
    #[schema(example = "verb")]
    pub part_of_speech: Option<String>,
    #[schema(example = "To move from one place to another")]
    pub definition: Option<String>,
    #[schema(example = "Nga ka noh")]
    pub example_pnar: Option<String>,
    #[schema(example = "I go home")]
    pub example_english: Option<String>,
    #[schema(example = 1)]
    pub difficulty_level: Option<i32>,
    #[schema(example = 10)]
    pub usage_frequency: Option<i32>,
    #[schema(example = "Common daily usage")]
    pub cultural_context: Option<String>,
    pub related_words: Option<serde_json::Value>,
    #[schema(example = "ka")]
    pub pronunciation: Option<String>,
    #[schema(example = "From Proto-Austroasiatic")]
    pub etymology: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Paginated response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationInfo {
    #[schema(example = 1)]
    pub page: i64,
    #[schema(example = 10)]
    pub per_page: i64,
    #[schema(example = 100)]
    pub total: i64,
    #[schema(example = 10)]
    pub pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, per_page: i64, total: i64) -> Self {
        let pages = (total + per_page - 1) / per_page; // Ceiling division

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
#[derive(Debug, Serialize, ToSchema)]
pub struct DictionaryPaginatedResponse {
    pub data: Vec<DictionaryEntryResponse>,
    pub pagination: PaginationInfo,
    pub timestamp: DateTime<Utc>,
}

impl DictionaryPaginatedResponse {
    pub fn new(data: Vec<DictionaryEntryResponse>, page: i64, per_page: i64, total: i64) -> Self {
        let pages = (total + per_page - 1) / per_page; // Ceiling division

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

/// Users paginated response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserPaginatedResponse {
    pub data: Vec<UserResponse>,
    pub pagination: PaginationInfo,
    pub timestamp: DateTime<Utc>,
}

impl UserPaginatedResponse {
    pub fn new(data: Vec<UserResponse>, page: i64, per_page: i64, total: i64) -> Self {
        let pages = (total + per_page - 1) / per_page; // Ceiling division

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

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "healthy")]
    pub status: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    pub timestamp: DateTime<Utc>,
    #[schema(example = "connected")]
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
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub user_id: Uuid,
    #[schema(example = "Hello world")]
    pub source_text: String,
    #[schema(example = "en")]
    pub source_language: String,
    #[schema(example = "pnar")]
    pub target_language: String,
    #[schema(example = "Kumno aiu")]
    pub translated_text: Option<String>,
    #[schema(example = "completed")]
    pub status: String,
    #[schema(example = "automatic")]
    pub translation_type: String,
    #[schema(example = 0.95)]
    pub confidence_score: Option<f64>,
    pub reviewed: bool,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User contribution response
#[derive(Debug, Serialize, ToSchema)]
pub struct ContributionResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub user_id: Uuid,
    #[schema(example = "dictionary_entry")]
    pub contribution_type: String,
    #[schema(example = "pnar_dictionary")]
    pub entity_type: String,
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub entity_id: Uuid,
    #[schema(example = "create")]
    pub action: String,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    #[schema(example = 10)]
    pub points_awarded: i32,
    #[schema(example = "approved")]
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Word usage analytics response
#[derive(Debug, Serialize, ToSchema)]
pub struct AnalyticsResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub word_id: Uuid,
    pub user_id: Option<Uuid>,
    #[schema(example = "search")]
    pub event_type: String,
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub timestamp: DateTime<Utc>,
    #[schema(example = "sess_12345")]
    pub session_id: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
