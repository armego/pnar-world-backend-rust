use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Role constants for backward compatibility
pub const SUPERADMIN: &str = "superadmin";
pub const ADMIN: &str = "admin";
pub const MODERATOR: &str = "moderator";
pub const TRANSLATOR: &str = "translator";
pub const CONTRIBUTOR: &str = "contributor";
pub const USER: &str = "user";

/// User role information
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoleInfo {
    #[schema(example = "admin")]
    pub role_id: &'static str,
    #[schema(example = "Administrator")]
    pub display_name: &'static str,
    #[schema(example = "Full system administration privileges")]
    pub description: &'static str,
    #[schema(example = 5)]
    pub hierarchy_level: u8,
    #[schema(example = true)]
    pub can_manage_users: bool,
    #[schema(example = true)]
    pub can_manage_dictionary: bool,
    #[schema(example = true)]
    pub can_manage_translations: bool,
}

/// User role enum for type safety
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    #[serde(rename = "superadmin")]
    SuperAdmin,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "translator")]
    Translator,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "user")]
    User,
}

/// Fixed application roles with their permissions
pub const APPLICATION_ROLES: [RoleInfo; 6] = [
    RoleInfo {
        role_id: SUPERADMIN,
        display_name: "Super Administrator",
        description: "Complete system control with all privileges",
        hierarchy_level: 6,
        can_manage_users: true,
        can_manage_dictionary: true,
        can_manage_translations: true,
    },
    RoleInfo {
        role_id: ADMIN,
        display_name: "Administrator",
        description: "System administration with user and content management",
        hierarchy_level: 5,
        can_manage_users: true,
        can_manage_dictionary: true,
        can_manage_translations: true,
    },
    RoleInfo {
        role_id: MODERATOR,
        display_name: "Moderator",
        description: "Content moderation and translation review",
        hierarchy_level: 4,
        can_manage_users: false,
        can_manage_dictionary: true,
        can_manage_translations: true,
    },
    RoleInfo {
        role_id: TRANSLATOR,
        display_name: "Translator",
        description: "Create and manage own translations",
        hierarchy_level: 3,
        can_manage_users: false,
        can_manage_dictionary: false,
        can_manage_translations: true,
    },
    RoleInfo {
        role_id: CONTRIBUTOR,
        display_name: "Contributor",
        description: "Submit translation suggestions and contributions",
        hierarchy_level: 2,
        can_manage_users: false,
        can_manage_dictionary: false,
        can_manage_translations: false,
    },
    RoleInfo {
        role_id: USER,
        display_name: "User",
        description: "Basic user with read access",
        hierarchy_level: 1,
        can_manage_users: false,
        can_manage_dictionary: false,
        can_manage_translations: false,
    },
];

/// Get role information by role_id
pub fn get_role_info(role_id: &str) -> Option<&RoleInfo> {
    APPLICATION_ROLES.iter().find(|role| role.role_id == role_id)
}

/// Check if a role exists
pub fn is_valid_role(role_id: &str) -> bool {
    APPLICATION_ROLES.iter().any(|role| role.role_id == role_id)
}

/// Get all available roles
pub fn get_all_roles() -> &'static [RoleInfo] {
    &APPLICATION_ROLES
}

/// Check if role_a has higher or equal hierarchy than role_b
pub fn has_hierarchy_level(role_a: &str, role_b: &str) -> bool {
    let level_a = get_role_info(role_a).map(|r| r.hierarchy_level).unwrap_or(0);
    let level_b = get_role_info(role_b).map(|r| r.hierarchy_level).unwrap_or(0);
    level_a >= level_b
}
