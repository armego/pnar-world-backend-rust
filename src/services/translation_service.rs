use crate::{
    constants::{defaults, error_messages, roles},
    dto::{responses::TranslationResponse, CreateTranslationRequest, UpdateTranslationRequest},
    error::AppError,
    utils::database,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

// Helper function to get user email
async fn get_user_email(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    database::get_user_email(pool, user_id).await
}

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
    .bind(request.source_language.as_deref().unwrap_or(defaults::DEFAULT_SOURCE_LANGUAGE))
    .bind(
        request
            .target_language
            .as_deref()
            .unwrap_or(defaults::DEFAULT_TARGET_LANGUAGE),
    )
    .bind(
        request
            .translation_type
            .as_deref()
            .unwrap_or(defaults::DEFAULT_TRANSLATION_TYPE),
    )
    .bind(&request.metadata.unwrap_or_else(|| serde_json::json!({})))
    .fetch_one(pool)
    .await?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: None, // For create, we don't join with users table
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_by_email: None, // Will be populated when querying with joins
        reviewed_at: record.get("reviewed_at"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn get_translation_request(
    pool: &PgPool,
    request_id: Uuid,
    user_id: Option<Uuid>,
    user_role: &str,
) -> Result<TranslationResponse, AppError> {
    // Build query based on user role and user_id
    let (query, bind_user_id) = if user_id.is_none() || user_role == roles::SUPERADMIN || user_role == roles::ADMIN {
        // Public access or admin access - can see any translation
        (r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as user_email, reviewer.email as reviewed_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        LEFT JOIN users reviewer ON tr.reviewed_by = reviewer.id
        WHERE tr.id = $1
        "#, false)
    } else {
        // User-specific access - can only see their own translations
        (r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as user_email, reviewer.email as reviewed_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        LEFT JOIN users reviewer ON tr.reviewed_by = reviewer.id
        WHERE tr.id = $1 AND tr.user_id = $2
        "#, true)
    };

    let mut query_builder = sqlx::query(query).bind(request_id);
    if bind_user_id {
        if let Some(uid) = user_id {
            query_builder = query_builder.bind(uid);
        }
    }

    let record = query_builder
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(error_messages::TRANSLATION_NOT_FOUND))?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: record.get("user_email"),
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_by_email: record.get("reviewed_by_email"),
        reviewed_at: record.get("reviewed_at"),
        metadata: record.get("metadata"),
        created_at: record.get("created_at"),
        updated_at: record.get("updated_at"),
    })
}

pub async fn list_translation_requests(
    pool: &PgPool,
    user_id: Option<Uuid>,
    user_role: &str,
    page: i64,
    per_page: i64,
) -> Result<Vec<TranslationResponse>, AppError> {
    let offset = (page - 1) * per_page;

    // Build query based on user role and user_id
    let (query, bind_user_id) = if user_id.is_none() || user_role == roles::SUPERADMIN || user_role == roles::ADMIN {
        // Public access or admin access - can see all translations
        (r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as user_email, reviewer.email as reviewed_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        LEFT JOIN users reviewer ON tr.reviewed_by = reviewer.id
        ORDER BY tr.created_at DESC
        LIMIT $1 OFFSET $2
        "#, false)
    } else {
        // User-specific access - can only see their own translations
        (r#"
        SELECT tr.id, tr.user_id, tr.source_text, tr.source_language, tr.target_language,
               tr.translated_text, tr.status, tr.translation_type, tr.confidence_score,
               tr.reviewed, tr.reviewed_by, tr.reviewed_at, tr.metadata, tr.created_at, tr.updated_at,
               u.email as user_email, reviewer.email as reviewed_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        LEFT JOIN users reviewer ON tr.reviewed_by = reviewer.id
        WHERE tr.user_id = $3
        ORDER BY tr.created_at DESC
        LIMIT $1 OFFSET $2
        "#, true)
    };

    let mut query_builder = sqlx::query(query).bind(per_page).bind(offset);
    if bind_user_id {
        if let Some(uid) = user_id {
            query_builder = query_builder.bind(uid);
        }
    }

    let records = query_builder.fetch_all(pool).await?;

    Ok(records
        .into_iter()
        .map(|record| TranslationResponse {
            id: record.get("id"),
            user_id: record.get("user_id"),
            user_email: record.get("user_email"),
            source_text: record.get("source_text"),
            source_language: record.get("source_language"),
            target_language: record.get("target_language"),
            translated_text: record.get("translated_text"),
            status: record.get("status"),
            translation_type: record.get("translation_type"),
            confidence_score: record.get("confidence_score"),
            reviewed: record.get("reviewed"),
            reviewed_by: record.get("reviewed_by"),
            reviewed_by_email: record.get("reviewed_by_email"),
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
    // Check if user can update this translation (owner only for regular users)
    let can_update = sqlx::query("SELECT id FROM translation_requests WHERE id = $1 AND user_id = $2")
        .bind(request_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .is_some();

    if !can_update {
        return Err(AppError::NotFound(error_messages::TRANSLATION_REQUEST_NOT_FOUND));
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
               u.email as user_email, reviewer.email as reviewed_by_email
        FROM translation_requests tr
        LEFT JOIN users u ON tr.user_id = u.id
        LEFT JOIN users reviewer ON tr.reviewed_by = reviewer.id
        WHERE tr.id = $1
        "#,
    )
    .bind(request_id)
    .fetch_one(pool)
    .await?;

    Ok(TranslationResponse {
        id: record.get("id"),
        user_id: record.get("user_id"),
        user_email: record.get("user_email"),
        source_text: record.get("source_text"),
        source_language: record.get("source_language"),
        target_language: record.get("target_language"),
        translated_text: record.get("translated_text"),
        status: record.get("status"),
        translation_type: record.get("translation_type"),
        confidence_score: record.get("confidence_score"),
        reviewed: record.get("reviewed"),
        reviewed_by: record.get("reviewed_by"),
        reviewed_by_email: record.get("reviewed_by_email"),
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
    // Check if user can delete this translation (owner only for regular users)
    let rows_affected = sqlx::query("DELETE FROM translation_requests WHERE id = $1 AND user_id = $2")
        .bind(request_id)
        .bind(user_id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound(error_messages::TRANSLATION_REQUEST_NOT_FOUND));
    }

    Ok(())
}

// Admin-only function to update any translation
pub async fn admin_update_translation_request(
    pool: &PgPool,
    request_id: Uuid,
    request: UpdateTranslationRequest,
) -> Result<TranslationResponse, AppError> {
    let query = r#"
        UPDATE translation_requests 
        SET translated_text = $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
        RETURNING id, source_text, translated_text, source_language, target_language, 
                  status, user_id, created_at, updated_at, translation_type, 
                  confidence_score, reviewed, reviewed_by, reviewed_at, metadata
    "#;
    
    let result = sqlx::query(query)
        .bind(&request.translated_text)
        .bind(request_id)
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) => {
            // Get user email
            let user_email = get_user_email(pool, row.get("user_id")).await?;
            
            // Get reviewer email if reviewed_by exists
            let reviewed_by_email = if let Some(reviewer_id) = row.try_get::<Option<Uuid>, _>("reviewed_by")? {
                Some(get_user_email(pool, reviewer_id).await?)
            } else {
                None
            };

            Ok(TranslationResponse {
                id: row.get("id"),
                source_text: row.get("source_text"),
                translated_text: row.get("translated_text"),
                source_language: row.get("source_language"),
                target_language: row.get("target_language"),
                status: row.get("status"),
                user_id: row.get("user_id"),
                user_email: Some(user_email),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                translation_type: row.get("translation_type"),
                confidence_score: row.get("confidence_score"),
                reviewed: row.get("reviewed"),
                reviewed_by: row.try_get("reviewed_by")?,
                reviewed_by_email,
                reviewed_at: row.try_get("reviewed_at")?,
                metadata: row.try_get("metadata")?,
            })
        },
        None => Err(AppError::NotFound(error_messages::TRANSLATION_REQUEST_NOT_FOUND)),
    }
}

// Admin-only function to delete any translation
pub async fn admin_delete_translation_request(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<(), AppError> {
    let rows_affected = sqlx::query("DELETE FROM translation_requests WHERE id = $1")
        .bind(request_id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound(error_messages::TRANSLATION_REQUEST_NOT_FOUND));
    }

    Ok(())
}
