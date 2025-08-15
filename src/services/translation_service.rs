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
        SELECT id, user_id, source_text, source_language, target_language,
               translated_text, status, translation_type, confidence_score,
               reviewed, reviewed_by, reviewed_at, metadata, created_at, updated_at
        FROM translation_requests 
        WHERE id = $1 AND user_id = $2
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
        SELECT id, user_id, source_text, source_language, target_language,
               translated_text, status, translation_type, confidence_score,
               reviewed, reviewed_by, reviewed_at, metadata, created_at, updated_at
        FROM translation_requests 
        WHERE user_id = $1
        ORDER BY created_at DESC
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
    request: UpdateTranslationRequest,
) -> Result<TranslationResponse, AppError> {
    let record = sqlx::query(
        r#"
        UPDATE translation_requests 
        SET 
            translated_text = COALESCE($3, translated_text),
            status = COALESCE($4, status),
            confidence_score = COALESCE($5, confidence_score),
            reviewed = COALESCE($6, reviewed),
            metadata = COALESCE($7, metadata),
            updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, source_text, source_language, target_language,
                  translated_text, status, translation_type, confidence_score,
                  reviewed, reviewed_by, reviewed_at, metadata, created_at, updated_at
        "#,
    )
    .bind(request_id)
    .bind(user_id)
    .bind(&request.translated_text)
    .bind(&request.status)
    .bind(request.confidence_score)
    .bind(request.reviewed)
    .bind(&request.metadata)
    .fetch_optional(pool)
    .await?;

    let record =
        record.ok_or_else(|| AppError::NotFound("Translation request not found".to_string()))?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
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
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM translation_requests WHERE id = $1 AND user_id = $2")
        .bind(request_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Translation request not found".to_string(),
        ));
    }

    Ok(())
}
