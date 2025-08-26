// Error message constants
pub const USER_NOT_FOUND: &str = "User not found";
pub const INVALID_CREDENTIALS: &str = "Invalid credentials";
pub const EMAIL_ALREADY_EXISTS: &str = "Email already exists";
pub const EMAIL_ALREADY_TAKEN: &str = "Email already taken";
pub const USER_ALREADY_EXISTS: &str = "User already exists";
pub const DICTIONARY_ENTRY_EXISTS: &str = "Dictionary entry already exists";
pub const PASSWORD_HASH_FAILED: &str = "Failed to hash password";
pub const INVALID_CURRENT_PASSWORD: &str = "Invalid current password";
pub const PASSWORD_PROCESSING_ERROR: &str = "Password processing error";
pub const UNAUTHORIZED: &str = "Unauthorized access";
pub const FORBIDDEN: &str = "Forbidden - insufficient permissions";
pub const INTERNAL_SERVER_ERROR: &str = "Internal server error";
pub const INVALID_TOKEN: &str = "Invalid or expired token";
pub const TRANSLATION_NOT_FOUND: &str = "Translation not found";
pub const TRANSLATION_REQUEST_NOT_FOUND: &str = "Translation request not found";
pub const DICTIONARY_ENTRY_NOT_FOUND: &str = "Dictionary entry not found";
pub const BOOK_NOT_FOUND: &str = "Book not found";
pub const CONTRIBUTION_NOT_FOUND: &str = "Contribution not found";
pub const ANALYTICS_NOT_FOUND: &str = "Analytics record not found";
pub const INVALID_INPUT: &str = "Invalid input provided";
pub const OPERATION_FAILED: &str = "Operation failed";
pub const YOU_CAN_ONLY_UPDATE_YOUR_OWN_ENTRIES: &str = "You can only update your own entries";
pub const YOU_CAN_ONLY_DELETE_YOUR_OWN_ENTRIES: &str = "You can only delete your own entries";

// Authentication and authorization messages
pub const USER_NOT_AUTHENTICATED: &str = "User not authenticated";
pub const MISSING_AUTH_TOKEN: &str = "Missing authentication token";
pub const SUPERADMIN_ACCESS_REQUIRED: &str = "Superadmin access required";
pub const ADMIN_ACCESS_REQUIRED: &str = "Admin access required";
pub const MODERATOR_ACCESS_REQUIRED: &str = "Moderator access required";
pub const CONTRIBUTOR_ACCESS_REQUIRED: &str = "Contributor access required";
pub const ROLE_ACCESS_REQUIRED: &str = "Insufficient role permissions for this operation";

// Profile access messages
pub const ONLY_OWN_PROFILE_OR_ADMIN: &str = "You can only access your own profile or you need admin privileges";
pub const ONLY_UPDATE_OWN_PROFILE_OR_ADMIN: &str = "You can only update your own profile or you need admin privileges";
pub const ONLY_UPDATE_OWN_PASSWORD_OR_ADMIN: &str = "You can only update your own password or you need admin privileges";
pub const ONLY_DELETE_OWN_ACCOUNT_OR_ADMIN: &str = "You can only delete your own account or you need admin privileges";
