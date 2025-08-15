use crate::{
    dto::{AuthApiResponse, AuthResponse, LoginRequest, RegisterRequest, UserApiResponse, UserResponse, ApiResponse},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::auth_service,
};
use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthApiResponse),
        (status = 400, description = "Invalid input data"),
        (status = 409, description = "User already exists")
    )
)]
#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    request: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;
    
    let auth_response = auth_service::register_user(&pool, request.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::new(auth_response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthApiResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;
    
    let auth_response = auth_service::login_user(&pool, request.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::new(auth_response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[post("/logout")]
pub async fn logout(_user: AuthenticatedUser) -> Result<HttpResponse, AppError> {
    // In a stateless JWT system, logout is typically handled client-side
    // For enhanced security, you might want to implement a token blacklist
    Ok(HttpResponse::Ok().json(ApiResponse::new("Logged out successfully")))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/profile",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserApiResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    )
)]
#[get("/profile")]
pub async fn profile(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_profile = auth_service::get_user_profile(&pool, user.id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::new(user_profile)))
}
