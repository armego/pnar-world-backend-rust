use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::OpenApi;
use utoipa::{openapi, Modify};

use crate::{
    constants::{alphabet::PnarCharacter, roles::{UserRole, RoleInfo}},
    dto::{
        analytics::{CreateAnalyticsRequest, UpdateAnalyticsRequest},
        auth::{LoginRequest, RefreshTokenRequest, RegisterRequest},
        book::{BookResponse, CreateBookRequest, UpdateBookRequest, BookQueryParams},
        contribution::{CreateContributionRequest, UpdateContributionRequest},
        dictionary::{
            CreateDictionaryEntryRequest, SearchDictionaryRequest, SearchType,
            UpdateDictionaryEntryRequest,
        },
        responses::{
            AnalyticsResponse, AnalyticsPaginatedResponse, AuthApiResponse, 
            AuthResponse, ContributionResponse, ContributionPaginatedResponse, DictionaryEntryResponse, 
            DictionaryPaginatedResponse, HealthResponse, PaginationInfo, SuccessResponse,
            TranslationResponse, TranslationPaginatedResponse, UserApiResponse, UserPaginatedResponse, 
            UserResponse,
        },
        translation::{CreateTranslationRequest, UpdateTranslationRequest},
        user::{
            AwardPointsRequest, CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest,
            UserQueryParams,
        },
    },
    handlers::alphabet::{ConvertTextRequest, ConvertTextResponse, ConversionDirection},
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
        crate::handlers::translation::create_translation,
        crate::handlers::translation::get_translation,
        crate::handlers::translation::list_translations,
        crate::handlers::translation::update_translation,
        crate::handlers::translation::delete_translation,
        crate::handlers::contribution::create_contribution,
        crate::handlers::contribution::get_contribution,
        crate::handlers::contribution::list_contributions,
        crate::handlers::contribution::update_contribution,
        crate::handlers::contribution::delete_contribution,
        crate::handlers::analytics::create_analytics,
        crate::handlers::analytics::create_anonymous_analytics,
        crate::handlers::analytics::get_analytics,
        crate::handlers::analytics::list_analytics,
        crate::handlers::analytics::update_analytics,
        crate::handlers::analytics::delete_analytics,
        crate::handlers::analytics::get_word_stats,
        crate::handlers::alphabet::list_alphabets,
        crate::handlers::alphabet::convert_text,
        crate::handlers::book::create_book,
        crate::handlers::book::get_book,
        crate::handlers::book::list_books,
        crate::handlers::book::update_book,
        crate::handlers::book::delete_book,
        crate::handlers::book::get_my_books,
        crate::handlers::roles::list_roles,
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

            // Translation DTOs
            CreateTranslationRequest,
            UpdateTranslationRequest,

            // Contribution DTOs
            CreateContributionRequest,
            UpdateContributionRequest,

            // Analytics DTOs
            CreateAnalyticsRequest,
            UpdateAnalyticsRequest,

            // Book DTOs
            CreateBookRequest,
            UpdateBookRequest,
            BookQueryParams,
            BookResponse,

            // Alphabet DTOs (read-only)
            PnarCharacter,
            ConvertTextRequest,
            ConvertTextResponse,
            ConversionDirection,
            
            // Roles (read-only)
            UserRole,
            RoleInfo,

            // Response DTOs
            SuccessResponse,
            AuthResponse,
            AuthApiResponse,
            UserResponse,
            UserApiResponse,
            DictionaryEntryResponse,
            DictionaryPaginatedResponse,
            UserPaginatedResponse,
            TranslationResponse,
            TranslationPaginatedResponse,
            ContributionResponse,
            ContributionPaginatedResponse,
            AnalyticsResponse,
            AnalyticsPaginatedResponse,
            HealthResponse,
            PaginationInfo,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "books", description = "Book management and discovery endpoints"),
        (name = "dictionary", description = "Dictionary management endpoints"),
        (name = "translations", description = "Translation request endpoints"),
        (name = "contributions", description = "User contribution endpoints"),
        (name = "analytics", description = "Word usage analytics endpoints"),
        (name = "alphabets", description = "Pnar alphabet character mappings"),
        (name = "roles", description = "User role information and permissions")
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
