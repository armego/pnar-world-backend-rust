use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

/// Get user email by ID - common utility function
pub async fn get_user_email(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    let email = sqlx::query_scalar("SELECT email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or("unknown@example.com".to_string());
    Ok(email)
}

/// Get user role by ID - common utility function  
pub async fn get_user_role(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    let role = sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or("user".to_string());
    Ok(role)
}
