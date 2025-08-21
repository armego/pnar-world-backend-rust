use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// User registration request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
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
}

/// User login request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@pnar.online")]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    #[schema(example = "securepassword123")]
    pub password: String,
}

/// Token refresh request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub refresh_token: String,
}
