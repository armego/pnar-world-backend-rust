use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pnar alphabet character response
#[derive(Debug, Serialize)]
pub struct PnarAlphabetResponse {
    pub id: Uuid,
    pub small: String,
    pub capital: String,
    pub kbf_small: String,
    pub kbf_capital: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

/// Request to create a new alphabet character mapping
#[derive(Debug, Deserialize)]
pub struct CreatePnarAlphabetRequest {
    pub id: Uuid,
    pub small: String,
    pub capital: String,
    pub kbf_small: String,
    pub kbf_capital: String,
    pub sort_order: i32,
}

/// Request to update an alphabet character mapping
#[derive(Debug, Deserialize)]
pub struct UpdatePnarAlphabetRequest {
    pub small: Option<String>,
    pub capital: Option<String>,
    pub kbf_small: Option<String>,
    pub kbf_capital: Option<String>,
    pub sort_order: Option<i32>,
}
