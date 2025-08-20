/// Authorization utility functions
use crate::{constants::{roles, error_messages}, error::AppError};
use uuid::Uuid;

/// Check if a user can modify a resource they created
/// - Superadmin and admin can modify any resource
/// - Regular users can only modify resources they created
pub fn can_modify_own_resource(user_role: &str, user_id: Uuid, created_by: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true,
        _ => created_by == Some(user_id),
    }
}

/// Check if a user can delete a resource
/// - Superadmin and admin can delete any resource
/// - Other users can only delete resources they created
pub fn can_delete_resource(user_role: &str, user_id: Uuid, created_by: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true,
        _ => created_by == Some(user_id),
    }
}

/// Check if a user can access another user's data
/// - Superadmin and admin can access any user's data
/// - Regular users can only access their own data
pub fn can_access_user_data(user_role: &str, user_id: Uuid, target_user_id: Uuid) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true,
        _ => user_id == target_user_id,
    }
}

/// Get role hierarchy level (higher number = more permissions)
pub fn get_role_level(role: &str) -> u8 {
    match role {
        roles::SUPERADMIN => 6,
        roles::ADMIN => 5,
        roles::MODERATOR => 4,
        roles::TRANSLATOR => 3,
        roles::CONTRIBUTOR => 2,
        roles::USER => 1,
        _ => 0, // Unknown role gets lowest access
    }
}

/// Check if a role has at least the required level
pub fn has_minimum_role_level(user_role: &str, required_role: &str) -> bool {
    get_role_level(user_role) >= get_role_level(required_role)
}

/// Validate that a user can perform an operation requiring a specific role
pub fn require_role(user_role: &str, required_role: &str) -> Result<(), AppError> {
    if !has_minimum_role_level(user_role, required_role) {
        return Err(AppError::Forbidden(error_messages::ROLE_ACCESS_REQUIRED));
    }
    Ok(())
}
