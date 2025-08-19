use crate::{
    dto::{responses::TranslationResponse, CreateTranslationRequest, UpdateTranslationRequest},
    error::AppError,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_translation_request(
    pool: &PgPool,
    user_id: Uuid,
    request: CreateTranslationRequest,
) -> Result<TranslationResponse, AppError> {
    let request_id = Uuid::new_v4();

    let record = sqlx::query(
        r#"
        INSERT INTO translation_requests (
            id, user_id, source_text, source_language, target_language,
            translation_type, metadata, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        RETURNING id, user_id, source_text, source_language, target_language,
                  translated_text, status, translation_type, confidence_score,
                  reviewed, reviewed_by, reviewed_at, metadata, created_at, updated_at
        "#,
    )
    .bind(request_id)
    .bind(user_id)
    .bind(&request.source_text)
    .bind(request.source_language.unwrap_or_else(|| "en".to_string()))
    .bind(
        request
            .target_language
            .unwrap_or_else(|| "pnar".to_string()),
    )
    .bind(
        request
            .translation_type
            .unwrap_or_else(|| "automatic".to_string()),
    )
    .bind(&request.metadata.unwrap_or_else(|| serde_json::json!({})))
    .fetch_one(pool)
    .await?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        created_by_email: None, // For create, we don't join with users table
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn get_translation_request(
    pool: &PgPool,
    request_id: Uuid,
    user_id: Uuid,
) -> Result<TranslationResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as created_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        WHERE tr.id = $1 AND tr.user_id = $2
        "#,
    )
    .bind(request_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let record =
        record.ok_or_else(|| AppError::NotFound("Translation request not found".to_string()))?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        created_by_email: record.get("created_by_email"),
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn list_translation_requests(
    pool: &PgPool,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<Vec<TranslationResponse>, AppError> {
    let offset = (page - 1) * per_page;

    let records = sqlx::query(
        r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as created_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        WHERE tr.user_id = $1
        ORDER BY tr.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|record| TranslationResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            created_by_email: record.get("created_by_email"),
            source_text: record.get("source_text"),
            source_language: record.get("source_language"),
            target_language: record.get("target_language"),
            translated_text: record.get("translated_text"),
            status: record.get("status"),
            translation_type: record.get("translation_type"),
            confidence_score: record.get("confidence_score"),
            reviewed: record.get("reviewed"),
            reviewed_by: record.get("reviewed_by"),
            reviewed_at: record.get("reviewed_at"),
            metadata: record.get("metadata"),
            created_at: record.get("created_at"),
            updated_at: record.get("updated_at"),
        })
        .collect())
}

pub async fn update_translation_request(
    pool: &PgPool,
    request_id: Uuid,
    user_id: Uuid,
    user_role: &str,
    request: UpdateTranslationRequest,
) -> Result<TranslationResponse, AppError> {
    // First, check if user can update this translation (owner or admin)
    let can_update = if user_role == "admin" {
        // Admin can update any translation
        sqlx::query("SELECT id FROM translation_requests WHERE id = $1")
            .bind(request_id)
            .fetch_optional(pool)
            .await?
            .is_some()
    } else {
        // Regular user can only update their own translations
        sqlx::query("SELECT id FROM translation_requests WHERE id = $1 AND user_id = $2")
            .bind(request_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .is_some()
    };

    if !can_update {
        return Err(AppError::NotFound("Translation request not found".to_string()));
    }

    // Update the translation
    sqlx::query(
        r#"
        UPDATE translation_requests 
        SET 
            translated_text = COALESCE($2, translated_text),
            status = COALESCE($3, status),
            confidence_score = COALESCE($4, confidence_score),
            reviewed = COALESCE($5, reviewed),
            metadata = COALESCE($6, metadata),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(request_id)
    .bind(&request.translated_text)
    .bind(&request.status)
    .bind(request.confidence_score)
    .bind(request.reviewed)
    .bind(&request.metadata)
    .execute(pool)
    .await?;

    // Fetch the updated record with user email
    let record = sqlx::query(
        r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as created_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        WHERE tr.id = $1
        "#,
    )
    .bind(request_id)
    .fetch_one(pool)
    .await?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        created_by_email: record.get("created_by_email"),
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_at: record.get("reviewed_at"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn delete_translation_request(
    pool: &PgPool,
    request_id: Uuid,
    user_id: Uuid,
    user_role: &str,
) -> Result<(), AppError> {
    // Check if user can delete this translation (owner or admin)
    let (query_str, bind_user_id) = if user_role == "admin" {
        ("DELETE FROM translation_requests WHERE id = $1", false)
    } else {
        ("DELETE FROM translation_requests WHERE id = $1 AND user_id = $2", true)
    };

    let mut query = sqlx::query(query_str).bind(request_id);
    
    if bind_user_id {
        query = query.bind(user_id);
    }
    
    let result = query.execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Translation request not found".to_string(),
        ));
    }

    Ok(())
}
