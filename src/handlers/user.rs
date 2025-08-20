use crate::{
    constants::error_messages,
    dto::{
        responses::{ApiResponse, SuccessResponse},
        user::{
            AwardPointsRequest, CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest,
            UserQueryParams,
        },
    },
    error::AppError,
    middleware::auth::{AdminUser, AuthenticatedUser},
    services::user_service,
};
use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use sqlx::PgPool;
use utoipa;
use uuid::Uuid;
use validator::Validate;

/// Create a new user
/// POST /api/v1/users
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserApiResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 409, description = "User already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    request: web::Json<CreateUserRequest>,
    _admin_user: AdminUser, // Only admins can create users
) -> Result<HttpResponse, AppError> {
    // Validate request
    request.validate()?;

    let user = user_service::create_user(&pool, request.into_inner()).await?;

    Ok(HttpResponse::Created().json(ApiResponse::new(user)))
}

/// Get user by ID
/// GET /api/v1/users/{id}
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserApiResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required or access to own profile"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{id}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    
    // Check if user can access this profile (admin or own profile)
    if !auth_user.can_access_user(user_id) {
        return Err(AppError::Forbidden(
            error_messages::ONLY_OWN_PROFILE_OR_ADMIN,
        ));
    }

    let user = user_service::get_user_by_id(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Get current user profile
/// GET /api/v1/users/me
#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current user profile retrieved successfully", body = UserApiResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    )
)]
#[get("/me")]
pub async fn get_current_user(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let current_user = user_service::get_user_by_id(&pool, auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(current_user)))
}

/// List users with pagination and filtering
/// GET /api/v1/users
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    params(UserQueryParams),
    responses(
        (status = 200, description = "Users retrieved successfully", body = UserPaginatedResponse),
        (status = 400, description = "Invalid query parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
pub async fn list_users(
    pool: web::Data<PgPool>,
    query: web::Query<UserQueryParams>,
    _admin_user: AdminUser, // Only admins can list all users
) -> Result<HttpResponse, AppError> {
    // Validate query parameters
    query.validate()?;

    let users = user_service::list_users(&pool, query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(users))
}

/// Update user
/// PUT /api/v1/users/{id}
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User updated successfully", body = UserApiResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required or access to own profile"),
        (status = 404, description = "User not found")
    )
)]
#[put("/{id}")]
pub async fn update_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateUserRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Validate request
    request.validate()?;

    // Check if user can update this profile (admin or own profile)
    if !auth_user.can_access_user(user_id) {
        return Err(AppError::Forbidden(
            error_messages::ONLY_UPDATE_OWN_PROFILE_OR_ADMIN,
        ));
    }

    let user = user_service::update_user(&pool, user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Update current user profile
/// PUT /api/v1/users/me
#[put("/me")]
pub async fn update_current_user(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateUserRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Validate request
    request.validate()?;

    let updated_user =
        user_service::update_user(&pool, auth_user.user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(updated_user)))
}

/// Update user password
/// PATCH /api/v1/users/{id}/password
#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/password",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdatePasswordRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Password updated successfully", body = SuccessResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required or access to own profile"),
        (status = 404, description = "User not found")
    )
)]
#[patch("/{id}/password")]
pub async fn update_user_password(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    request: web::Json<UpdatePasswordRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Validate request
    request.validate()?;

    // Check if user can update this password (admin or own profile)
    if !auth_user.can_access_user(user_id) {
        return Err(AppError::Forbidden(
            error_messages::ONLY_UPDATE_OWN_PASSWORD_OR_ADMIN,
        ));
    }

    user_service::update_user_password(&pool, user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        "Password updated successfully".to_string(),
    )))
}

/// Update current user password
/// PATCH /api/v1/users/me/password
#[patch("/me/password")]
pub async fn update_current_user_password(
    pool: web::Data<PgPool>,
    request: web::Json<UpdatePasswordRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Validate request
    request.validate()?;

    user_service::update_user_password(&pool, auth_user.user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        "Password updated successfully".to_string(),
    )))
}

/// Delete user (soft delete)
/// DELETE /api/v1/users/{id}
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = SuccessResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required or access to own profile"),
        (status = 404, description = "User not found")
    )
)]
#[delete("/{id}")]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Check if user can delete this account (admin or own account)
    if !auth_user.can_access_user(user_id) {
        return Err(AppError::Forbidden(
            error_messages::ONLY_DELETE_OWN_ACCOUNT_OR_ADMIN,
        ));
    }

    user_service::delete_user(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        "User deleted successfully".to_string(),
    )))
}

/// Delete current user account (soft delete)
/// DELETE /api/v1/users/me
#[delete("/me")]
pub async fn delete_current_user(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    user_service::delete_user(&pool, auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        "Account deleted successfully".to_string(),
    )))
}

/// Award points to user
/// POST /api/v1/users/{id}/points
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/points",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = AwardPointsRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Points awarded successfully", body = UserApiResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 404, description = "User not found")
    )
)]
#[post("/{id}/points")]
pub async fn award_points(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    request: web::Json<AwardPointsRequest>,
    _admin_user: AdminUser, // Only admins can award points
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Validate request
    request.validate()?;

    let user = user_service::award_points(&pool, user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Verify user email
/// POST /api/v1/users/{id}/verify-email
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/verify-email",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Email verified successfully", body = UserApiResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 404, description = "User not found")
    )
)]
#[post("/{id}/verify-email")]
pub async fn verify_email(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _admin_user: AdminUser, // Only admins can verify emails manually
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    let user = user_service::verify_email(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Get user by email
/// GET /api/v1/users/email/{email}
#[utoipa::path(
    get,
    path = "/api/v1/users/email/{email}",
    tag = "users",
    params(
        ("email" = String, Path, description = "User email")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserApiResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 404, description = "User not found")
    )
)]
#[get("/email/{email}")]
pub async fn get_user_by_email(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    _admin_user: AdminUser, // Only admins can search users by email
) -> Result<HttpResponse, AppError> {
    let email = path.into_inner();

    let user = user_service::get_user_by_email(&pool, &email).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}
