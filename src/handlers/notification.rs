use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{
        notification::{
            CreateNotificationRequest, NotificationQueryParams, UpdateNotificationRequest,
            MarkNotificationReadRequest,
        },
        responses::{ApiResponse, SuccessResponse},
    },
    error::AppError,
    middleware::auth::AuthenticatedUser,
    services::notification_service,
};

/// Create a new notification
#[post("")]
pub async fn create_notification(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    request: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;

    let notification = notification_service::create_notification(
        &pool,
        user.user_id,
        request.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse::new(notification)))
}

/// Get a notification by ID
#[get("/{id}")]
pub async fn get_notification(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();

    let notification = notification_service::get_notification(
        &pool,
        notification_id,
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(notification)))
}

/// List notifications for the current user
#[get("")]
pub async fn list_notifications(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    query: web::Query<NotificationQueryParams>,
) -> Result<HttpResponse, AppError> {
    let notifications = notification_service::list_notifications(
        &pool,
        user.user_id,
        query.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(notifications))
}

/// Update a notification
#[put("/{id}")]
pub async fn update_notification(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdateNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();
    request.validate()?;

    let notification = notification_service::update_notification(
        &pool,
        notification_id,
        user.user_id,
        request.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(notification)))
}

/// Mark notification as read/unread
#[patch("/{id}/read")]
pub async fn mark_notification_read(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<MarkNotificationReadRequest>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();

    let notification = notification_service::mark_notification_read(
        &pool,
        notification_id,
        user.user_id,
        request.read,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(notification)))
}

/// Delete a notification
#[delete("/{id}")]
pub async fn delete_notification(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();

    notification_service::delete_notification(
        &pool,
        notification_id,
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new("Notification deleted successfully".to_string())))
}

/// Mark all notifications as read
#[patch("/mark-all-read")]
pub async fn mark_all_notifications_read(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let count = notification_service::mark_all_notifications_read(
        &pool,
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        format!("Marked {} notifications as read", count),
    )))
}

/// Get unread notifications count
#[get("/unread-count")]
pub async fn get_unread_count(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let count = notification_service::get_unread_count(
        &pool,
        user.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(serde_json::json!({
        "unread_count": count
    }))))
}
