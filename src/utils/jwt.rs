use crate::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: i64,    // Expiry time
    pub iat: i64,    // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(24);

        Self {
            sub: user_id.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, AppError> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))
    }
}

pub fn generate_token(user_id: Uuid) -> Result<String, AppError> {
    let claims = Claims::new(user_id);
    let secret = get_jwt_secret();
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))
}

pub fn generate_refresh_token(user_id: Uuid) -> Result<String, AppError> {
    let now = Utc::now();
    let expiry = now + Duration::days(30); // 30 days for refresh token

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiry.timestamp(),
        iat: now.timestamp(),
    };
    
    let secret = get_jwt_secret();
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| AppError::Internal(format!("Failed to generate refresh token: {}", e)))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string())
}
