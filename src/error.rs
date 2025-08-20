use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use crate::constants::error_messages;

/// Application-wide error types
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Authentication(&'static str),

    #[error("Authorization error: {0}")]
    Authorization(&'static str),

    #[error("Unauthorized: {0}")]
    Unauthorized(&'static str),

    #[error("Forbidden: {0}")]
    Forbidden(&'static str),

    #[error("Validation error: {0}")]
    Validation(String), // Keep String for complex validation messages

    #[error("Not found: {0}")]
    NotFound(&'static str),

    #[error("Conflict: {0}")]
    Conflict(&'static str),

    #[error("Internal server error: {0}")]
    Internal(String), // Keep String for complex internal errors

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error: {0}")]
    PasswordHash(String), // Keep String for complex error details
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_code, message) = match self {
            AppError::Authentication(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "AUTH_ERROR",
                self.to_string(),
            ),
            AppError::Authorization(_) => (
                actix_web::http::StatusCode::FORBIDDEN,
                "AUTHORIZATION_ERROR",
                self.to_string(),
            ),
            AppError::Unauthorized(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                self.to_string(),
            ),
            AppError::Forbidden(_) => (
                actix_web::http::StatusCode::FORBIDDEN,
                "FORBIDDEN",
                self.to_string(),
            ),
            AppError::Validation(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                self.to_string(),
            ),
            AppError::NotFound(_) => (
                actix_web::http::StatusCode::NOT_FOUND,
                "NOT_FOUND",
                self.to_string(),
            ),
            AppError::Conflict(_) => (
                actix_web::http::StatusCode::CONFLICT,
                "CONFLICT",
                self.to_string(),
            ),
            AppError::Database(_) | AppError::Internal(_) | AppError::Config(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                error_messages::INTERNAL_SERVER_ERROR.to_string(),
            ),
            AppError::Jwt(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "TOKEN_ERROR",
                error_messages::INVALID_TOKEN.to_string(),
            ),
            AppError::PasswordHash(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "PASSWORD_ERROR",
                error_messages::PASSWORD_PROCESSING_ERROR.to_string(),
            ),
        };

        HttpResponse::build(status).json(json!({
            "error": {
                "code": error_code,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }))
    }
}

/// Result type alias for the application
pub type AppResult<T> = Result<T, AppError>;

/// Convert argon2 errors to AppError
impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHash(err.to_string())
    }
}

/// Convert validation errors to AppError
impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = err
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "{}: {}",
                        field,
                        error.message.as_ref().unwrap_or(&"Invalid value".into())
                    )
                })
            })
            .collect();

        AppError::Validation(error_messages.join("; "))
    }
}

/// Convert IO errors to AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IO error: {}", err))
    }
}

/// Convert migrate errors to AppError
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        AppError::Internal(format!("Migration error: {}", err))
    }
}
