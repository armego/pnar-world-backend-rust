use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Create user request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@pnar.online")]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[schema(example = "securepassword123")]
    pub password: String,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Full name must be between 2 and 100 characters"
    ))]
    #[schema(example = "John Doe")]
    pub full_name: Option<String>,

    #[validate(url(message = "Invalid URL format"))]
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,

    #[validate(length(
        min = 2,
        max = 20,
        message = "Role must be between 2 and 20 characters"
    ))]
    #[schema(example = "user")]
    pub role: Option<String>,

    #[validate(length(max = 500, message = "Bio must be less than 500 characters"))]
    #[schema(example = "Language enthusiast")]
    pub bio: Option<String>,

    #[validate(length(
        min = 2,
        max = 10,
        message = "Preferred language must be 2-10 characters"
    ))]
    #[schema(example = "en")]
    pub preferred_language: Option<String>,

    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

/// Update user request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@pnar.online")]
    pub email: Option<String>,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Full name must be between 2 and 100 characters"
    ))]
    #[schema(example = "Jane Doe")]
    pub full_name: Option<String>,

    #[validate(url(message = "Invalid URL format"))]
    #[schema(example = "https://example.com/newavatar.jpg")]
    pub avatar_url: Option<String>,

    #[validate(length(
        min = 2,
        max = 20,
        message = "Role must be between 2 and 20 characters"
    ))]
    #[schema(example = "admin")]
    pub role: Option<String>,

    #[validate(length(max = 500, message = "Bio must be less than 500 characters"))]
    #[schema(example = "Updated bio")]
    pub bio: Option<String>,

    #[validate(length(
        min = 2,
        max = 10,
        message = "Preferred language must be 2-10 characters"
    ))]
    #[schema(example = "es")]
    pub preferred_language: Option<String>,

    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub is_email_verified: Option<bool>,
}

/// Update user password request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 8, message = "Current password is required"))]
    #[schema(example = "currentpassword123")]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters long"))]
    #[schema(example = "newpassword123")]
    pub new_password: String,
}

/// User query parameters for listing users
#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct UserQueryParams {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    #[schema(example = 1)]
    pub page: Option<i64>,

    #[validate(range(min = 1, max = 100, message = "Per page must be between 1 and 100"))]
    #[schema(example = 10)]
    pub per_page: Option<i64>,

    #[schema(example = "user")]
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_email_verified: Option<bool>,
    #[schema(example = "john")]
    pub search: Option<String>, // Search in email or full_name
}

/// Award points request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AwardPointsRequest {
    #[validate(range(min = -1000, max = 1000, message = "Points must be between -1000 and 1000"))]
    #[schema(example = 50)]
    pub points: i32,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Reason is required and must be less than 255 characters"
    ))]
    #[schema(example = "Good translation work")]
    pub reason: String,
}
