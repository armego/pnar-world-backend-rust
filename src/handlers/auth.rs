use crate::{
    dto::{responses::AuthApiResponse, ApiResponse, LoginRequest, RegisterRequest},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::{auth_service, user_service},
};
use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    request: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let auth_response = auth_service::register_user(&pool, request.into_inner()).await?;

    Ok(HttpResponse::Created().json(AuthApiResponse::new(auth_response)))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let auth_response = auth_service::login_user(&pool, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(AuthApiResponse::new(auth_response)))
}

)]
#[post("/logout")]
pub async fn logout(_user: AuthenticatedUser) -> Result<HttpResponse, AppError> {
    // In a stateless JWT system, logout is typically handled client-side
    // For enhanced security, you might want to implement a token blacklist
    Ok(HttpResponse::Ok().json(ApiResponse::new("Logged out successfully")))
}

)]
#[get("/profile")]
pub async fn profile(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_profile = user_service::get_user_by_id(&pool, user.user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(user_profile)))
}
