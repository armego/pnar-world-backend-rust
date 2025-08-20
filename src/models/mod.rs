// Basic models and types used throughout the application
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum UserRole {
    Admin,
    Moderator,
    Translator,
    Contributor,
    User,
}

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => UserRole::Admin,
            "moderator" => UserRole::Moderator,
            "translator" => UserRole::Translator,
            "contributor" => UserRole::Contributor,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        use crate::constants::roles;
        match role {
            UserRole::Admin => roles::ADMIN.to_string(),
            UserRole::Moderator => roles::MODERATOR.to_string(),
            UserRole::Translator => roles::TRANSLATOR.to_string(),
            UserRole::Contributor => roles::CONTRIBUTOR.to_string(),
            UserRole::User => roles::USER.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub translation_points: i32,
    pub bio: Option<String>,
    pub preferred_language: String,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
