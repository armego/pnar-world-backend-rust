use crate::{
    constants::error_messages,
    dto::{responses::{AnalyticsResponse, AnalyticsPaginatedResponse}, CreateAnalyticsRequest, UpdateAnalyticsRequest},
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
            id, user_id, word_id, usage_type, session_id,
            context_data, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, user_id, word_id, usage_type, session_id,
                  context_data, created_at
        "#,
    )
    .bind(analytics_id)
    .bind(user_id)
    .bind(&request.word_id)
    .bind(&request.usage_type)
    .bind(&request.session_id)
    .bind(&request.context_data.unwrap_or_else(|| serde_json::json!({})))
    .fetch_one(pool)
    .await?;

    Ok(AnalyticsResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For create, we don't join with users table
        word_id: record.get("word_id"),
        usage_type: record.get("usage_type"),
        timestamp: record.get("created_at"), // Use created_at as timestamp
        session_id: record.get("session_id"),
        context_data: record.get("context_data"),
        created_at: record.get("created_at"),
    })
}

pub async fn get_analytics_record(
    pool: &PgPool,
    analytics_id: Uuid,
) -> Result<AnalyticsResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT id, user_id, word_id, usage_type, session_id,
               context_data, created_at
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
        usage_type: record.get("usage_type"),
        timestamp: record.get("created_at"), // Use created_at as timestamp
        session_id: record.get("session_id"),
        context_data: record.get("context_data"),
        created_at: record.get("created_at"),
    })
}

pub async fn list_analytics_records(
    pool: &PgPool,
    user_id: Option<Uuid>,
    word_id: Option<Uuid>,
    usage_type: Option<&str>,
    page: i64,
    per_page: i64,
) -> Result<AnalyticsPaginatedResponse, AppError> {
    let offset = (page - 1) * per_page;

    // First, get the total count
    let mut count_query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT COUNT(*) FROM word_usage_analytics w
        "#,
    );

    let has_user_id = user_id.is_some();
    let has_word_id = word_id.is_some();
    let has_usage_type = usage_type.is_some();

    if has_user_id || has_word_id || has_usage_type {
        count_query_builder.push(" WHERE ");
        let mut separated = count_query_builder.separated(" AND ");
        if let Some(uid) = user_id {
            separated.push("w.user_id = ");
            separated.push_bind(uid);
        }
        if let Some(wid) = word_id {
            separated.push("w.word_id = ");
            separated.push_bind(wid);
        }
        if let Some(ut) = usage_type {
            separated.push("w.usage_type = ");
            separated.push_bind(ut);
        }
    }

    let count_query = count_query_builder.build();
    let total_result = count_query.fetch_one(pool).await?;
    let total: i64 = total_result.get(0);

    // Then get the paginated records
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            w.id, w.user_id, u.email as user_email, w.word_id, w.usage_type, 
            w.created_at, w.session_id, w.context_data, w.created_at
        FROM word_usage_analytics w
        LEFT JOIN users u ON w.user_id = u.id
        "#,
    );

    if has_user_id || has_word_id || has_usage_type {
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
        if let Some(ut) = usage_type {
            separated.push("w.usage_type = ");
            separated.push_bind(ut);
        }
    }

    query_builder.push(" ORDER BY w.created_at DESC LIMIT ");
    query_builder.push_bind(per_page);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build();
    let records = query.fetch_all(pool).await?;

    let items: Vec<AnalyticsResponse> = records
        .into_iter()
        .map(|record| AnalyticsResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            user_email: record.get("user_email"),
            word_id: record.get("word_id"),
            usage_type: record.get("usage_type"),
            timestamp: record.get("created_at"), // Use created_at as timestamp
            session_id: record.get("session_id"),
            context_data: record.get("context_data"),
            created_at: record.get("created_at"),
        })
        .collect();

    Ok(AnalyticsPaginatedResponse::new(items, page, per_page, total))
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
            context_data = COALESCE($2, context_data)
        WHERE id = $1
        RETURNING id, user_id, word_id, usage_type, session_id,
                  context_data, created_at
        "#,
    )
    .bind(analytics_id)
    .bind(&request.context_data)
    .fetch_optional(pool)
    .await?;

    let record =
        record.ok_or_else(|| AppError::NotFound(error_messages::ANALYTICS_NOT_FOUND))?;

    Ok(AnalyticsResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For update, we don't join with users table
        word_id: record.get("word_id"),
        usage_type: record.get("usage_type"),
        timestamp: record.get("created_at"), // Use created_at as timestamp
        session_id: record.get("session_id"),
        context_data: record.get("context_data"),
        created_at: record.get("created_at"),
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
                usage_type,
                COUNT(*) as count,
                DATE_TRUNC('day', timestamp) as date
            FROM word_usage_analytics 
            WHERE word_id = $1 AND user_id = $2
            GROUP BY usage_type, DATE_TRUNC('day', timestamp) 
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
                usage_type,
                COUNT(*) as count,
                DATE_TRUNC('day', timestamp) as date
            FROM word_usage_analytics 
            WHERE word_id = $1
            GROUP BY usage_type, DATE_TRUNC('day', timestamp) 
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
                "usage_type": record.get::<String, _>("usage_type"),
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
