use utoipa::OpenApi;
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, Http};
use utoipa::{Modify, openapi};

use crate::dto::{
    auth::{LoginRequest, RegisterRequest, RefreshTokenRequest},
    dictionary::{CreateDictionaryEntryRequest, UpdateDictionaryEntryRequest, SearchDictionaryRequest, SearchType},
    responses::{AuthResponse, AuthApiResponse, UserResponse, UserApiResponse, DictionaryEntryResponse, PaginatedResponse, HealthResponse, PaginationInfo, SuccessResponse},
    user::{CreateUserRequest, UpdateUserRequest, UpdatePasswordRequest, UserQueryParams, AwardPointsRequest},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::profile,
        crate::handlers::user::create_user,
        crate::handlers::user::get_user,
        crate::handlers::user::get_current_user,
        crate::handlers::user::list_users,
        crate::handlers::user::update_user,
        crate::handlers::user::delete_user,
        crate::handlers::dictionary::create_entry,
        crate::handlers::dictionary::get_entry,
        crate::handlers::dictionary::list_entries,
        crate::handlers::dictionary::search_entries,
        crate::handlers::dictionary::update_entry,
        crate::handlers::dictionary::delete_entry,
        crate::handlers::dictionary::verify_entry,
    ),
    components(
        schemas(
            // Auth DTOs
            LoginRequest,
            RegisterRequest,
            RefreshTokenRequest,
            
            // User DTOs
            CreateUserRequest,
            UpdateUserRequest,
            UpdatePasswordRequest,
            UserQueryParams,
            AwardPointsRequest,
            
            // Dictionary DTOs
            CreateDictionaryEntryRequest,
            UpdateDictionaryEntryRequest,
            SearchDictionaryRequest,
            SearchType,
            
            // Response DTOs
            SuccessResponse,
            AuthApiResponse,
            UserApiResponse,
            AuthResponse,
            UserResponse,
            DictionaryEntryResponse,
            PaginatedResponse<UserResponse>,
            PaginatedResponse<DictionaryEntryResponse>,
            PaginationInfo,
            HealthResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "dictionary", description = "Dictionary management endpoints")
    ),
    info(
        title = "Pnar World Dictionary API",
        version = "0.1.0",
        description = "A modern web service for Pnar language translation and dictionary management",
        contact(
            name = "Pnar World Team",
            email = "unix121@protonmail.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    servers(
        (url = "http://localhost:8000", description = "Local development server"),
        (url = "https://api.pnarworld.com", description = "Production server")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}
