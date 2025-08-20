use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Pnar alphabet character response
#[derive(Debug, Serialize, ToSchema)]
pub struct PnarAlphabetResponse {
    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub id: Uuid,
    #[schema(example = "æ")]
    pub small: String,
    #[schema(example = "Æ")]
    pub capital: String,
    #[schema(example = "se")]
    pub kbf_small: String,
    #[schema(example = "Ae")]
    pub kbf_capital: String,
    #[schema(example = 6)]
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

/// Request to create a new alphabet character mapping
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePnarAlphabetRequest {
    #[schema(example = "æ")]
    pub small: String,
    #[schema(example = "Æ")]
    pub capital: String,
    #[schema(example = "se")]
    pub kbf_small: String,
    #[schema(example = "Ae")]
    pub kbf_capital: String,
    #[schema(example = 6)]
    pub sort_order: i32,
}

/// Request to update an alphabet character mapping
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePnarAlphabetRequest {
    #[schema(example = "æ")]
    pub small: Option<String>,
    #[schema(example = "Æ")]
    pub capital: Option<String>,
    #[schema(example = "se")]
    pub kbf_small: Option<String>,
    #[schema(example = "Ae")]
    pub kbf_capital: Option<String>,
    #[schema(example = 6)]
    pub sort_order: Option<i32>,
}
