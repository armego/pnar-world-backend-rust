use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Request to create a new user contribution
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateContributionRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Contribution type must be between 1 and 50 characters"
    ))]
    #[schema(example = "dictionary_entry")]
    pub contribution_type: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Entity type must be between 1 and 50 characters"
    ))]
    #[schema(example = "pnar_dictionary")]
    pub entity_type: String,

    #[schema(example = "f47ac10b-58cc-4372-a567-0e02b2c3d479")]
    pub entity_id: Uuid,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Action must be between 1 and 50 characters"
    ))]
    #[schema(example = "create")]
    pub action: String,

    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,

    #[schema(example = 10)]
    pub points_awarded: Option<i32>,
}

/// Request to update an existing contribution
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateContributionRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Status must be between 1 and 50 characters"
    ))]
    #[schema(example = "approved")]
    pub status: Option<String>,
}
