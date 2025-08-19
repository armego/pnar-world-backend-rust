use crate::{
    dto::{
        responses::{AuthResponse, UserResponse},
        LoginRequest, RegisterRequest,
    },
    error::AppError,
    utils::jwt,
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn register_user(
    pool: &PgPool,
    request: RegisterRequest,
) -> Result<AuthResponse, AppError> {
    // Check if user already exists
    let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&request.email)
        .fetch_optional(pool)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();

    // Create user
    let user_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password, full_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        "#,
    )
    .bind(user_id)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&request.full_name)
    .execute(pool)
    .await?;

    // Generate JWT token
    let token = jwt::generate_token(user_id)?;
    let refresh_token = jwt::generate_refresh_token(user_id)?;

    // Get the created user for response
    let user_record = sqlx::query(
        r#"SELECT 
            id, email, full_name, avatar_url, role, translation_points, 
            bio, preferred_language, settings, is_active, is_email_verified, 
            created_at, updated_at 
        FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let user_response = UserResponse {
        id: user_record.get("id"),
        email: user_record.get("email"),
        full_name: user_record.get("full_name"),
        avatar_url: user_record.get("avatar_url"),
        role: user_record.get("role"),
        translation_points: user_record.get("translation_points"),
        bio: user_record.get("bio"),
        preferred_language: user_record.get("preferred_language"),
        settings: user_record.get("settings"),
        is_active: user_record.get("is_active"),
        is_email_verified: user_record.get("is_email_verified"),
        created_at: user_record.get("created_at"),
        updated_at: user_record.get("updated_at"),
    };

    Ok(AuthResponse {
        user: user_response,
        access_token: token,
        refresh_token,
        expires_in: 86400, // 24 hours
    })
}

pub async fn login_user(pool: &PgPool, request: LoginRequest) -> Result<AuthResponse, AppError> {
    // Get user from database
    let user_record = sqlx::query("SELECT id, password FROM users WHERE email = $1")
        .bind(&request.email)
        .fetch_optional(pool)
        .await?;

    let user_record =
        user_record.ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let password: String = user_record.get("password");
    let parsed_hash = PasswordHash::new(&password)
        .map_err(|e| AppError::Internal(format!("Failed to parse password hash: {}", e)))?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let user_id: Uuid = user_record.get("id");

    // Generate JWT token
    let token = jwt::generate_token(user_id)?;
    let refresh_token = jwt::generate_refresh_token(user_id)?;

    // Get user details for response
    let user_details = sqlx::query(
        r#"SELECT 
            id, email, full_name, avatar_url, role, translation_points, 
            bio, preferred_language, settings, is_active, is_email_verified, 
            created_at, updated_at 
        FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let user_response = UserResponse {
        id: user_details.get("id"),
        email: user_details.get("email"),
        full_name: user_details.get("full_name"),
        avatar_url: user_details.get("avatar_url"),
        role: user_details.get("role"),
        translation_points: user_details.get("translation_points"),
        bio: user_details.get("bio"),
        preferred_language: user_details.get("preferred_language"),
        settings: user_details.get("settings"),
        is_active: user_details.get("is_active"),
        is_email_verified: user_details.get("is_email_verified"),
        created_at: user_details.get("created_at"),
        updated_at: user_details.get("updated_at"),
    };

    Ok(AuthResponse {
        user: user_response,
        access_token: token,
        refresh_token,
        expires_in: 86400, // 24 hours
    })
}

pub async fn get_user_profile(pool: &PgPool, user_id: Uuid) -> Result<UserResponse, AppError> {
    let user_record = sqlx::query(
        r#"
        SELECT 
            id, email, full_name, avatar_url, role, translation_points, 
            bio, preferred_language, settings, is_active, is_email_verified, 
            created_at, updated_at
        FROM users 
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let user_record =
        user_record.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(UserResponse {
        id: user_record.get("id"),
        email: user_record.get("email"),
        full_name: user_record.get("full_name"),
        avatar_url: user_record.get("avatar_url"),
        role: user_record.get("role"),
        translation_points: user_record.get("translation_points"),
        bio: user_record.get("bio"),
        preferred_language: user_record.get("preferred_language"),
        settings: user_record.get("settings"),
        is_active: user_record.get("is_active"),
        is_email_verified: user_record.get("is_email_verified"),
        created_at: user_record.get("created_at"),
        updated_at: user_record.get("updated_at"),
    })
}
