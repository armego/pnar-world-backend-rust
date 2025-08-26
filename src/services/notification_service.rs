use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    dto::{
        notification::{
            CreateNotificationRequest, NotificationResponse, UpdateNotificationRequest,
            NotificationQueryParams,
        },
        responses::PaginatedResponse,
    },
    error::AppError,
};

pub async fn create_notification(
    pool: &PgPool,
    user_id: Uuid,
    request: CreateNotificationRequest,
) -> Result<NotificationResponse, AppError> {
    let notification_id = Uuid::new_v4();

    let record = sqlx::query(
        r#"
        INSERT INTO notifications (id, user_id, type, title, message, data, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, type, title, message, data, read, read_at, created_at, expires_at
        "#,
    )
    .bind(notification_id)
    .bind(user_id)
    .bind(&request.r#type)
    .bind(&request.title)
    .bind(&request.message)
    .bind(&request.data.unwrap_or(serde_json::json!({})))
    .bind(request.expires_at)
    .fetch_one(pool)
    .await?;

    Ok(NotificationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        r#type: record.get("type"),
        title: record.get("title"),
        message: record.get("message"),
        data: record.get("data"),
        read: record.get("read"),
        read_at: record.get("read_at"),
        created_at: record.get("created_at"),
        expires_at: record.get("expires_at"),
    })
}

pub async fn get_notification(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<NotificationResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT id, user_id, type, title, message, data, read, read_at, created_at, expires_at
        FROM notifications
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(notification_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Notification not found"))?;

    Ok(NotificationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        r#type: record.get("type"),
        title: record.get("title"),
        message: record.get("message"),
        data: record.get("data"),
        read: record.get("read"),
        read_at: record.get("read_at"),
        created_at: record.get("created_at"),
        expires_at: record.get("expires_at"),
    })
}

pub async fn list_notifications(
    pool: &PgPool,
    user_id: Uuid,
    params: NotificationQueryParams,
) -> Result<PaginatedResponse<NotificationResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // Build the WHERE clause
    let mut where_conditions = vec!["user_id = $1".to_string()];
    let mut param_count = 1;

    if let Some(_notification_type) = &params.r#type {
        param_count += 1;
        where_conditions.push(format!("type = ${}", param_count));
    }

    if let Some(_read_status) = params.read {
        param_count += 1;
        where_conditions.push(format!("read = ${}", param_count));
    }

    if !params.include_expired.unwrap_or(false) {
        where_conditions.push("(expires_at IS NULL OR expires_at > NOW())".to_string());
    }

    let where_clause = where_conditions.join(" AND ");

    // Count total records
    let count_query = format!(
        "SELECT COUNT(*) FROM notifications WHERE {}",
        where_clause
    );

    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query).bind(user_id);
    
    if let Some(notification_type) = &params.r#type {
        count_query_builder = count_query_builder.bind(notification_type);
    }
    
    if let Some(read_status) = params.read {
        count_query_builder = count_query_builder.bind(read_status);
    }

    let total = count_query_builder.fetch_one(pool).await?;

    // Fetch records
    param_count += 1;
    let limit_param = param_count;
    param_count += 1;
    let offset_param = param_count;

    let data_query = format!(
        r#"
        SELECT id, user_id, type, title, message, data, read, read_at, created_at, expires_at
        FROM notifications
        WHERE {}
        ORDER BY created_at DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_clause, limit_param, offset_param
    );

    let mut data_query_builder = sqlx::query(&data_query).bind(user_id);
    
    if let Some(notification_type) = &params.r#type {
        data_query_builder = data_query_builder.bind(notification_type);
    }
    
    if let Some(read_status) = params.read {
        data_query_builder = data_query_builder.bind(read_status);
    }
    
    data_query_builder = data_query_builder.bind(per_page).bind(offset);

    let records = data_query_builder.fetch_all(pool).await?;

    let notifications: Vec<NotificationResponse> = records
        .into_iter()
        .map(|record| NotificationResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            r#type: record.get("type"),
            title: record.get("title"),
            message: record.get("message"),
            data: record.get("data"),
            read: record.get("read"),
            read_at: record.get("read_at"),
            created_at: record.get("created_at"),
            expires_at: record.get("expires_at"),
        })
        .collect();

    Ok(PaginatedResponse::new(notifications, page, per_page, total))
}

pub async fn update_notification(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
    request: UpdateNotificationRequest,
) -> Result<NotificationResponse, AppError> {
    // Check if notification exists and belongs to user
    let _existing = sqlx::query(
        "SELECT id FROM notifications WHERE id = $1 AND user_id = $2"
    )
    .bind(notification_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Notification not found"))?;

    // Build dynamic update query
    let mut update_fields = Vec::new();
    let mut param_count = 1;

    if request.r#type.is_some() {
        param_count += 1;
        update_fields.push(format!("type = ${}", param_count));
    }

    if request.title.is_some() {
        param_count += 1;
        update_fields.push(format!("title = ${}", param_count));
    }

    if request.message.is_some() {
        param_count += 1;
        update_fields.push(format!("message = ${}", param_count));
    }

    if request.data.is_some() {
        param_count += 1;
        update_fields.push(format!("data = ${}", param_count));
    }

    if request.expires_at.is_some() {
        param_count += 1;
        update_fields.push(format!("expires_at = ${}", param_count));
    }

    if update_fields.is_empty() {
        return Err(AppError::Validation("No fields to update".to_string()));
    }

    let update_query = format!(
        r#"
        UPDATE notifications 
        SET {}
        WHERE id = $1
        RETURNING id, user_id, type, title, message, data, read, read_at, created_at, expires_at
        "#,
        update_fields.join(", ")
    );

    let mut query_builder = sqlx::query(&update_query).bind(notification_id);

    if let Some(ref r#type) = request.r#type {
        query_builder = query_builder.bind(r#type);
    }

    if let Some(ref title) = request.title {
        query_builder = query_builder.bind(title);
    }

    if let Some(ref message) = request.message {
        query_builder = query_builder.bind(message);
    }

    if let Some(ref data) = request.data {
        query_builder = query_builder.bind(data);
    }

    if let Some(expires_at) = request.expires_at {
        query_builder = query_builder.bind(expires_at);
    }

    let record = query_builder.fetch_one(pool).await?;

    Ok(NotificationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        r#type: record.get("type"),
        title: record.get("title"),
        message: record.get("message"),
        data: record.get("data"),
        read: record.get("read"),
        read_at: record.get("read_at"),
        created_at: record.get("created_at"),
        expires_at: record.get("expires_at"),
    })
}

pub async fn mark_notification_read(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
    read: bool,
) -> Result<NotificationResponse, AppError> {
    let read_at = if read { Some(Utc::now()) } else { None };

    let record = sqlx::query(
        r#"
        UPDATE notifications 
        SET read = $3, read_at = $4
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, type, title, message, data, read, read_at, created_at, expires_at
        "#,
    )
    .bind(notification_id)
    .bind(user_id)
    .bind(read)
    .bind(read_at)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Notification not found"))?;

    Ok(NotificationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        r#type: record.get("type"),
        title: record.get("title"),
        message: record.get("message"),
        data: record.get("data"),
        read: record.get("read"),
        read_at: record.get("read_at"),
        created_at: record.get("created_at"),
        expires_at: record.get("expires_at"),
    })
}

pub async fn delete_notification(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "DELETE FROM notifications WHERE id = $1 AND user_id = $2"
    )
    .bind(notification_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Notification not found"));
    }

    Ok(())
}

pub async fn mark_all_notifications_read(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<i64, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE notifications 
        SET read = true, read_at = NOW()
        WHERE user_id = $1 AND read = false
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

pub async fn delete_expired_notifications(pool: &PgPool) -> Result<i64, AppError> {
    let result = sqlx::query(
        "DELETE FROM notifications WHERE expires_at IS NOT NULL AND expires_at < NOW()"
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

pub async fn get_unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM notifications 
        WHERE user_id = $1 AND read = false 
        AND (expires_at IS NULL OR expires_at > NOW())
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(count)
}
