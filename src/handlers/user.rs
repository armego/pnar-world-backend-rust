use crate::{
    dto::{
        responses::{ApiResponse, SuccessResponse},
        user::{
            AwardPointsRequest, CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest,
            UserQueryParams,
        },
    },
    error::AppError,
    middleware::auth::AuthenticatedUser,
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
        (status = 409, description = "User already exists")
    )
)]
#[post("")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    request: web::Json<CreateUserRequest>,
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
        (status = 404, description = "User not found")
    )
)]
#[get("/{id}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
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
        (status = 200, description = "Users retrieved successfully", body = PaginatedResponse<UserResponse>),
        (status = 400, description = "Invalid query parameters")
    )
)]
#[get("")]
pub async fn list_users(
    pool: web::Data<PgPool>,
    query: web::Query<UserQueryParams>,
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
        (status = 403, description = "Forbidden"),
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

    // Check if user is updating their own profile or has admin role
    if user_id != auth_user.user_id {
        // Here you would check if the authenticated user has admin role
        // For now, we'll allow any authenticated user to update any profile
        // In production, you should implement proper role-based access control
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

    // Only allow users to update their own password
    if user_id != auth_user.user_id {
        return Err(AppError::Forbidden(
            "You can only update your own password".to_string(),
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
        (status = 403, description = "Forbidden"),
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

    // Check if user is deleting their own account or has admin role
    if user_id != auth_user.user_id {
        // Here you would check if the authenticated user has admin role
        // For now, we'll prevent users from deleting other accounts
        return Err(AppError::Forbidden(
            "You can only delete your own account".to_string(),
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
#[post("/{id}/points")]
pub async fn award_points(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    request: web::Json<AwardPointsRequest>,
    _auth_user: AuthenticatedUser, // TODO: Check if user has admin role
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Validate request
    request.validate()?;

    // TODO: Implement role-based access control
    // Only admins should be able to award points

    let user = user_service::award_points(&pool, user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Verify user email
/// POST /api/v1/users/{id}/verify-email
#[post("/{id}/verify-email")]
pub async fn verify_email(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    _auth_user: AuthenticatedUser, // TODO: Check if user has admin role
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // TODO: Implement role-based access control
    // Only admins should be able to verify emails manually

    let user = user_service::verify_email(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}

/// Get user by email
/// GET /api/v1/users/email/{email}
#[get("/email/{email}")]
pub async fn get_user_by_email(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    _auth_user: AuthenticatedUser, // TODO: Check if user has appropriate role
) -> Result<HttpResponse, AppError> {
    let email = path.into_inner();

    // TODO: Implement role-based access control
    // This endpoint should be restricted to admins or specific roles

    let user = user_service::get_user_by_email(&pool, &email).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user)))
}
