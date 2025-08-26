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

    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            w.id, w.user_id, u.email as user_email, w.word_id, w.event_type, 
            w.timestamp, w.session_id, w.metadata, w.created_at, w.updated_at
        FROM word_usage_analytics w
        LEFT JOIN users u ON w.user_id = u.id
        "#,
    );

    let has_user_id = user_id.is_some();
    let has_word_id = word_id.is_some();
    let has_event_type = event_type.is_some();

    if has_user_id || has_word_id || has_event_type {
        query_builder.push(" WHERE ");
        let mut separated = query_builder.separated(" AND ");
        if let Some(uid) = user_id {
            separated.push("w.user_id = ");
            separated.push_bind(uid);
        }
        if let Some(wid) = word_id {
            separated.push("w.word_id = ");
            separated.push_bind(wid);
        }
        if let Some(et) = event_type {
            separated.push("w.event_type = ");
            separated.push_bind(et);
        }
    }

    query_builder.push(" ORDER BY w.timestamp DESC LIMIT ");
    query_builder.push_bind(per_page);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build();

    let records = query.fetch_all(pool).await?;

    Ok(records
        .into_iter()
        .map(|record| AnalyticsResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            user_email: record.get("user_email"),
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
