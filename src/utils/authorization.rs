/// Authorization utility functions
use crate::{constants::{roles, error_messages}, error::AppError};
use uuid::Uuid;

/// Check if a user can manage another user based on role hierarchy
/// - Superadmin: can manage all users (admin, contributor, user)
/// - Admin: can CRUD users below their rank (contributor, user) and view same rank (admin)
/// - Contributor/User: cannot manage users
pub fn can_manage_user(manager_role: &str, target_role: &str) -> bool {
    let manager_level = get_role_level(manager_role);
    let target_level = get_role_level(target_role);
    
    match manager_role {
        roles::SUPERADMIN => true, // Superadmin can manage all
        roles::ADMIN => target_level < manager_level, // Admin can CRUD below rank only
        _ => false, // Contributor and User cannot manage users
    }
}

/// Check if a user can view another user's profile
/// - Superadmin: can view all users
/// - Admin: can view users of same rank and below
/// - Contributor/User: can only view their own profile
pub fn can_view_user(viewer_role: &str, viewer_id: Uuid, target_role: &str, target_id: Uuid) -> bool {
    let viewer_level = get_role_level(viewer_role);
    let target_level = get_role_level(target_role);
    
    match viewer_role {
        roles::SUPERADMIN => true, // Superadmin can view all
        roles::ADMIN => target_level <= viewer_level, // Admin can view same rank and below
        _ => viewer_id == target_id, // Others can only view their own profile
    }
}

/// Check if a user can modify translations
/// - Superadmin/Admin: can modify any translation
/// - Contributor: can only modify their own translations and read others
/// - User: can only read translations
pub fn can_modify_translation(user_role: &str, user_id: Uuid, translation_owner: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true, // Can modify any translation
        roles::CONTRIBUTOR => {
            // Contributors can only modify their own translations
            translation_owner == Some(user_id)
        },
        _ => false, // Users cannot modify translations
    }
}

/// Check if a user can read translations
/// All authenticated users can read translations
pub fn can_read_translation(_user_role: &str) -> bool {
    true // All users can read translations
}

/// Check if a user can delete translations
/// - Superadmin/Admin: can delete any translation
/// - Contributor: can only delete their own translations
/// - User: cannot delete translations
pub fn can_delete_translation(user_role: &str, user_id: Uuid, translation_owner: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true, // Can delete any translation
        roles::CONTRIBUTOR => {
            // Contributors can only delete their own translations
            translation_owner == Some(user_id)
        },
        _ => false, // Users cannot delete translations
    }
}

/// Check if a user can access user management features
/// - Superadmin/Admin: can access user management
/// - Contributor/User: cannot access user management
pub fn can_access_user_management(user_role: &str) -> bool {
    matches!(user_role, roles::SUPERADMIN | roles::ADMIN)
}

/// Get roles that a user can assign to others based on hierarchy
/// - Superadmin: can assign all roles
/// - Admin: can only assign contributor and user roles
/// - Others: cannot assign roles
pub fn get_assignable_roles(user_role: &str) -> Vec<&'static str> {
    match user_role {
        roles::SUPERADMIN => vec![
            roles::SUPERADMIN,
            roles::ADMIN,
            roles::MODERATOR,
            roles::CONTRIBUTOR,
            roles::USER,
        ],
        roles::ADMIN => vec![
            roles::CONTRIBUTOR,
            roles::USER,
        ],
        _ => vec![], // Contributors and users cannot assign roles
    }
}

/// Check if a user can assign a specific role
pub fn can_assign_role(manager_role: &str, target_role: &str) -> bool {
    get_assignable_roles(manager_role).contains(&target_role)
}

/// Legacy function - Check if a user can modify a resource they created
/// - Superadmin and admin can modify any resource
/// - Regular users can only modify resources they created
pub fn can_modify_own_resource(user_role: &str, user_id: Uuid, created_by: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true,
        _ => created_by == Some(user_id),
    }
}

/// Legacy function - Check if a user can delete a resource
/// - Superadmin and admin can delete any resource
/// - Other users can only delete resources they created
pub fn can_delete_resource(user_role: &str, user_id: Uuid, created_by: Option<Uuid>) -> bool {
    match user_role {
        roles::SUPERADMIN | roles::ADMIN => true,
        _ => created_by == Some(user_id),
    }
}

/// Legacy function - Check if a user can access another user's data
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
        roles::SUPERADMIN => 5,
        roles::ADMIN => 4,
        roles::MODERATOR => 3,
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
