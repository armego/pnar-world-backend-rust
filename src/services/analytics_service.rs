use crate::{
    constants::error_messages,
    dto::{responses::AnalyticsResponse, CreateAnalyticsRequest, UpdateAnalyticsRequest},
    error::AppError,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_analytics_record(
    pool: &PgPool,
    user_id: Option<Uuid>,
    request: CreateAnalyticsRequest,
) -> Result<AnalyticsResponse, AppError> {
    let analytics_id = Uuid::new_v4();

    let record = sqlx::query(
        r#"
        INSERT INTO word_usage_analytics (
            id, user_id, word_id, event_type, timestamp, session_id,
            metadata, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        RETURNING id, user_id, word_id, event_type, timestamp, session_id,
                  metadata, created_at, updated_at
        "#,
    )
    .bind(analytics_id)
    .bind(user_id)
    .bind(&request.word_id)
    .bind(&request.event_type)
    .bind(&request.timestamp)
    .bind(&request.session_id)
    .bind(&request.metadata.unwrap_or_else(|| serde_json::json!({})))
    .fetch_one(pool)
    .await?;

    Ok(AnalyticsResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For create, we don't join with users table
        word_id: record.get("word_id"),
        event_type: record.get("event_type"),
        timestamp: record.get("timestamp"),
        session_id: record.get("session_id"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn get_analytics_record(
    pool: &PgPool,
    analytics_id: Uuid,
) -> Result<AnalyticsResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT id, user_id, word_id, event_type, timestamp, session_id,
               metadata, created_at, updated_at
        FROM word_usage_analytics 
        WHERE id = $1
        "#,
    )
    .bind(analytics_id)
    .fetch_optional(pool)
    .await?;

    let record =
        record.ok_or_else(|| AppError::NotFound(error_messages::ANALYTICS_NOT_FOUND))?;

    Ok(AnalyticsResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For single record, we don't join with users table
        word_id: record.get("word_id"),
        event_type: record.get("event_type"),
        timestamp: record.get("timestamp"),
        session_id: record.get("session_id"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn list_analytics_records(
    pool: &PgPool,
    user_id: Option<Uuid>,
    word_id: Option<Uuid>,
    event_type: Option<&str>,
    page: i64,
    per_page: i64,
) -> Result<Vec<AnalyticsResponse>, AppError> {
    let offset = (page - 1) * per_page;

    // Use separate queries based on parameters to avoid complex dynamic binding
    let records = match (user_id, word_id, event_type) {
        (Some(uid), Some(wid), Some(et)) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE user_id = $1 AND word_id = $2 AND event_type = $3
                ORDER BY timestamp DESC
                LIMIT $4 OFFSET $5
                "#,
            )
            .bind(uid)
            .bind(wid)
            .bind(et)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (Some(uid), Some(wid), None) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE user_id = $1 AND word_id = $2
                ORDER BY timestamp DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(uid)
            .bind(wid)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (Some(uid), None, Some(et)) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE user_id = $1 AND event_type = $2
                ORDER BY timestamp DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(uid)
            .bind(et)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (Some(uid), None, None) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE user_id = $1
                ORDER BY timestamp DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(uid)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (None, Some(wid), Some(et)) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE word_id = $1 AND event_type = $2
                ORDER BY timestamp DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(wid)
            .bind(et)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (None, Some(wid), None) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE word_id = $1
                ORDER BY timestamp DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(wid)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (None, None, Some(et)) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                WHERE event_type = $1
                ORDER BY timestamp DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(et)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
        (None, None, None) => {
            sqlx::query(
                r#"
                SELECT id, user_id, word_id, event_type, timestamp, session_id,
                       metadata, created_at, updated_at
                FROM word_usage_analytics 
                ORDER BY timestamp DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(records
        .into_iter()
        .map(|record| AnalyticsResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            user_email: None, // For list, we don't join with users table by default
            word_id: record.get("word_id"),
            event_type: record.get("event_type"),
            timestamp: record.get("timestamp"),
            session_id: record.get("session_id"),
            metadata: record.get("metadata"),
            created_at: record.get("created_at"),
            updated_at: record.get("updated_at"),
        })
        .collect())
}

pub async fn update_analytics_record(
    pool: &PgPool,
    analytics_id: Uuid,
    request: UpdateAnalyticsRequest,
) -> Result<AnalyticsResponse, AppError> {
    let record = sqlx::query(
        r#"
        UPDATE word_usage_analytics 
        SET 
            metadata = COALESCE($2, metadata),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, word_id, event_type, timestamp, session_id,
                  metadata, created_at, updated_at
        "#,
    )
    .bind(analytics_id)
    .bind(&request.metadata)
    .fetch_optional(pool)
    .await?;

    let record =
        record.ok_or_else(|| AppError::NotFound(error_messages::ANALYTICS_NOT_FOUND))?;

    Ok(AnalyticsResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For update, we don't join with users table
        word_id: record.get("word_id"),
        event_type: record.get("event_type"),
        timestamp: record.get("timestamp"),
        session_id: record.get("session_id"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn delete_analytics_record(pool: &PgPool, analytics_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM word_usage_analytics WHERE id = $1")
        .bind(analytics_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(error_messages::ANALYTICS_NOT_FOUND));
    }

    Ok(())
}

pub async fn get_word_usage_stats(
    pool: &PgPool,
    word_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<serde_json::Value, AppError> {
    let records = if let Some(uid) = user_id {
        sqlx::query(
            r#"
            SELECT 
                event_type,
                COUNT(*) as count,
                DATE_TRUNC('day', timestamp) as date
            FROM word_usage_analytics 
            WHERE word_id = $1 AND user_id = $2
            GROUP BY event_type, DATE_TRUNC('day', timestamp) 
            ORDER BY date DESC
            "#,
        )
        .bind(word_id)
        .bind(uid)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT 
                event_type,
                COUNT(*) as count,
                DATE_TRUNC('day', timestamp) as date
            FROM word_usage_analytics 
            WHERE word_id = $1
            GROUP BY event_type, DATE_TRUNC('day', timestamp) 
            ORDER BY date DESC
            "#,
        )
        .bind(word_id)
        .fetch_all(pool)
        .await?
    };

    let stats: Vec<serde_json::Value> = records
        .into_iter()
        .map(|record| {
            serde_json::json!({
                "event_type": record.get::<String, _>("event_type"),
                "count": record.get::<i64, _>("count"),
                "date": record.get::<chrono::DateTime<chrono::Utc>, _>("date")
            })
        })
        .collect();

    Ok(serde_json::json!({
        "word_id": word_id,
        "statistics": stats
    }))
}
