use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

/// Request to create a new user contribution
#[derive(Debug, Deserialize, Validate)]
pub struct CreateContributionRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Contribution type must be between 1 and 50 characters"
    ))]
    pub contribution_type: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Entity type must be between 1 and 50 characters"
    ))]
    pub entity_type: String,

    pub entity_id: Uuid,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Action must be between 1 and 50 characters"
    ))]
    pub action: String,

    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,

    pub points_awarded: Option<i32>,
}

/// Request to update an existing contribution
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContributionRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Status must be between 1 and 50 characters"
    ))]
    pub status: Option<String>,
}
