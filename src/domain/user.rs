use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String, // Usually not serialized to responses
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

pub struct UserBuilder {
    id: Option<Uuid>,
    email: Option<String>,
    password: Option<String>,
    full_name: Option<String>,
    avatar_url: Option<String>,
    role: Option<String>,
    translation_points: Option<i32>,
    bio: Option<String>,
    preferred_language: Option<String>,
    settings: Option<serde_json::Value>,
    is_active: Option<bool>,
    is_email_verified: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            email: None,
            password: None,
            full_name: None,
            avatar_url: None,
            role: None,
            translation_points: None,
            bio: None,
            preferred_language: None,
            settings: None,
            is_active: None,
            is_email_verified: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    pub fn full_name(mut self, full_name: Option<String>) -> Self {
        self.full_name = full_name;
        self
    }

    pub fn avatar_url(mut self, avatar_url: Option<String>) -> Self {
        self.avatar_url = avatar_url;
        self
    }

    pub fn role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }

    pub fn translation_points(mut self, points: i32) -> Self {
        self.translation_points = Some(points);
        self
    }

    pub fn bio(mut self, bio: Option<String>) -> Self {
        self.bio = bio;
        self
    }

    pub fn preferred_language(mut self, lang: String) -> Self {
        self.preferred_language = Some(lang);
        self
    }

    pub fn settings(mut self, settings: serde_json::Value) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn is_email_verified(mut self, is_verified: bool) -> Self {
        self.is_email_verified = Some(is_verified);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> User {
        User {
            id: self.id.expect("id is required"),
            email: self.email.expect("email is required"),
            password: self.password.expect("password is required"),
            full_name: self.full_name,
            avatar_url: self.avatar_url,
            role: self.role.unwrap_or_else(|| "user".to_string()),
            translation_points: self.translation_points.unwrap_or(0),
            bio: self.bio,
            preferred_language: self.preferred_language.unwrap_or_else(|| "en".to_string()),
            settings: self.settings.unwrap_or_else(|| serde_json::json!({})),
            is_active: self.is_active.unwrap_or(true),
            is_email_verified: self.is_email_verified.unwrap_or(false),
            created_at: self.created_at.expect("created_at is required"),
            updated_at: self.updated_at.expect("updated_at is required"),
        }
    }
}
