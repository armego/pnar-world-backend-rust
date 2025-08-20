use crate::{
    constants::error_messages,
    dto::{
        responses::{DictionaryEntryResponse, DictionaryPaginatedResponse},
        CreateDictionaryEntryRequest, SearchDictionaryRequest, UpdateDictionaryEntryRequest,
    },
    error::AppError,
    utils::{analytics_tracker::AnalyticsTracker, database},
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

// Helper function to build DictionaryEntryResponse with email addresses
async fn build_dictionary_response(
    pool: &PgPool,
    entry_record: &sqlx::postgres::PgRow,
) -> Result<DictionaryEntryResponse, AppError> {
    let created_by: Option<Uuid> = entry_record.get("created_by");
    let updated_by: Option<Uuid> = entry_record.get("updated_by");
    let verified_by: Option<Uuid> = entry_record.get("verified_by");
    
    // Get creator email
    let created_by_email = if let Some(creator_id) = created_by {
        Some(database::get_user_email(pool, creator_id).await?)
    } else {
        None
    };

    // Get updater email
    let updated_by_email = if let Some(updater_id) = updated_by {
        Some(database::get_user_email(pool, updater_id).await?)
    } else {
        None
    };

    // Get verifier email
    let verified_by_email = if let Some(verifier_id) = verified_by {
        Some(database::get_user_email(pool, verifier_id).await?)
    } else {
        None
    };    Ok(DictionaryEntryResponse {
        id: entry_record.get("id"),
        pnar_word: entry_record.get("pnar_word"),
        pnar_word_kbf: entry_record.get("pnar_word_kbf"),
        english_word: entry_record.get("english_word"),
        part_of_speech: entry_record.get("part_of_speech"),
        definition: entry_record.get("definition"),
        example_pnar: entry_record.get("example_pnar"),
        example_english: entry_record.get("example_english"),
        difficulty_level: entry_record.get("difficulty_level"),
        usage_frequency: entry_record.get("usage_frequency"),
        cultural_context: entry_record.get("cultural_context"),
        related_words: entry_record.get("related_words"),
        pronunciation: entry_record.get("pronunciation"),
        etymology: entry_record.get("etymology"),
        verified: entry_record.get("verified"),
        created_at: entry_record.get("created_at"),
        updated_at: entry_record.get("updated_at"),
        created_by,
        created_by_email,
        updated_by,
        updated_by_email,
        verified_by,
        verified_by_email,
        verified_at: entry_record.get("verified_at"),
    })
}

pub async fn create_entry(
    pool: &PgPool,
    author_id: Uuid,
    request: CreateDictionaryEntryRequest,
) -> Result<DictionaryEntryResponse, AppError> {
    let entry_id = Uuid::new_v4();

    // Check if pnar_word already exists
    let existing = sqlx::query("SELECT id FROM pnar_dictionary WHERE pnar_word = $1")
        .bind(&request.pnar_word)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::Internal(format!(
            "Pnar word '{}' {}",
            request.pnar_word, error_messages::DICTIONARY_ENTRY_EXISTS
        )));
    }

    let entry_record = sqlx::query(
        r#"
        INSERT INTO pnar_dictionary (
            id, pnar_word, pnar_word_kbf, english_word, part_of_speech, definition,
            example_pnar, example_english, difficulty_level, usage_frequency,
            cultural_context, related_words, pronunciation, etymology,
            created_by, created_at, updated_at, verified
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, NOW(), NOW(), $16
        )
        RETURNING id, pnar_word, pnar_word_kbf, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by, updated_by, verified_by, verified_at
        "#
    )
    .bind(entry_id)
    .bind(&request.pnar_word)
    .bind(&request.pnar_word_kbf)
    .bind(&request.english_word)
    .bind(&request.part_of_speech)
    .bind(&request.definition)
    .bind(&request.example_pnar)
    .bind(&request.example_english)
    .bind(request.difficulty_level.unwrap_or(1))
    .bind(request.usage_frequency.unwrap_or(0))
    .bind(&request.cultural_context)
    .bind(&request.related_words)
    .bind(&request.pronunciation)
    .bind(&request.etymology)
    .bind(author_id)
    .bind(false) // verified default
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Internal(format!("Dictionary entry with pnar_word '{}' already exists", request.pnar_word));
            }
        }
        AppError::Database(e)
    })?;

    let response = build_dictionary_response(pool, &entry_record).await?;

    // Track contribution analytics
    let points = AnalyticsTracker::calculate_contribution_points("dictionary_entry", "create");
    let new_value = serde_json::json!({
        "pnar_word": request.pnar_word,
        "english_word": request.english_word,
        "part_of_speech": request.part_of_speech,
        "definition": request.definition
    });

    if let Err(e) = AnalyticsTracker::track_contribution(
        pool,
        author_id,
        "dictionary_entry",
        "pnar_dictionary",
        entry_id,
        "create",
        None, // No previous value for creation
        Some(new_value),
        points,
    ).await {
        tracing::warn!("Failed to track contribution analytics: {}", e);
    }

    Ok(response)
}

pub async fn get_entry(
    pool: &PgPool, 
    entry_id: Uuid, 
    user_id: Option<Uuid>,
    session_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<DictionaryEntryResponse, AppError> {
    let entry_record = sqlx::query(
        r#"
        SELECT id, pnar_word, pnar_word_kbf, english_word, part_of_speech, definition,
               example_pnar, example_english, difficulty_level, usage_frequency,
               cultural_context, related_words, pronunciation, etymology,
               verified, created_at, updated_at, created_by, updated_by, verified_by, verified_at
        FROM pnar_dictionary 
        WHERE id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(pool)
    .await?;

    let entry_record =
        entry_record.ok_or_else(|| AppError::NotFound(error_messages::DICTIONARY_ENTRY_NOT_FOUND))?;

    let response = build_dictionary_response(pool, &entry_record).await?;

    // Track word usage analytics
    if let Err(e) = AnalyticsTracker::track_word_usage(
        pool,
        entry_id,
        user_id,
        "lookup",
        session_id,
        ip_address,
        user_agent,
        Some(serde_json::json!({
            "pnar_word": response.pnar_word,
            "english_word": response.english_word
        }))
    ).await {
        tracing::warn!("Failed to track word usage analytics: {}", e);
    }

    Ok(response)
}

pub async fn list_entries(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<DictionaryPaginatedResponse, AppError> {
    let offset = (page - 1) * per_page;

    let entries = sqlx::query(
        r#"
        SELECT d.id, d.pnar_word, d.pnar_word_kbf, d.english_word, d.part_of_speech, d.definition,
               d.example_pnar, d.example_english, d.difficulty_level, d.usage_frequency,
               d.cultural_context, d.related_words, d.pronunciation, d.etymology,
               d.verified, d.created_at, d.updated_at, d.created_by, d.updated_by, d.verified_by, d.verified_at,
               creator.email as created_by_email, updater.email as updated_by_email, verifier.email as verified_by_email
        FROM pnar_dictionary d
        LEFT JOIN users creator ON d.created_by = creator.id
        LEFT JOIN users updater ON d.updated_by = updater.id
        LEFT JOIN users verifier ON d.verified_by = verifier.id
        ORDER BY d.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total_result = sqlx::query("SELECT COUNT(*) FROM pnar_dictionary")
        .fetch_one(pool)
        .await?;
    let total: i64 = total_result.get(0);

    let items: Vec<DictionaryEntryResponse> = entries
        .into_iter()
        .map(|record| DictionaryEntryResponse {
            id: record.get("id"),
            pnar_word: record.get("pnar_word"),
            pnar_word_kbf: record.get("pnar_word_kbf"),
            english_word: record.get("english_word"),
            part_of_speech: record.get("part_of_speech"),
            definition: record.get("definition"),
            example_pnar: record.get("example_pnar"),
            example_english: record.get("example_english"),
            difficulty_level: record.get("difficulty_level"),
            usage_frequency: record.get("usage_frequency"),
            cultural_context: record.get("cultural_context"),
            related_words: record.get("related_words"),
            pronunciation: record.get("pronunciation"),
            etymology: record.get("etymology"),
            verified: record.get("verified"),
            created_at: record.get("created_at"),
            updated_at: record.get("updated_at"),
            created_by: record.get("created_by"),
            created_by_email: record.get("created_by_email"),
            updated_by: record.get("updated_by"),
            updated_by_email: record.get("updated_by_email"),
            verified_by: record.get("verified_by"),
            verified_by_email: record.get("verified_by_email"),
            verified_at: record.get("verified_at"),
        })
        .collect();

    Ok(DictionaryPaginatedResponse::new(
        items, page, per_page, total,
    ))
}

pub async fn search_entries(
    pool: &PgPool,
    request: SearchDictionaryRequest,
    user_id: Option<Uuid>,
    session_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<Vec<DictionaryEntryResponse>, AppError> {
    let query = format!("%{}%", request.query);

    let entries = sqlx::query(
        r#"
        SELECT d.id, d.pnar_word, d.pnar_word_kbf, d.english_word, d.part_of_speech, d.definition,
               d.example_pnar, d.example_english, d.difficulty_level, d.usage_frequency,
               d.cultural_context, d.related_words, d.pronunciation, d.etymology,
               d.verified, d.created_at, d.updated_at, d.created_by, d.updated_by, d.verified_by, d.verified_at,
               creator.email as created_by_email, updater.email as updated_by_email, verifier.email as verified_by_email
        FROM pnar_dictionary d
        LEFT JOIN users creator ON d.created_by = creator.id
        LEFT JOIN users updater ON d.updated_by = updater.id
        LEFT JOIN users verifier ON d.verified_by = verifier.id
        WHERE d.pnar_word ILIKE $1 OR d.english_word ILIKE $1 OR d.definition ILIKE $1
        ORDER BY 
            CASE WHEN d.pnar_word ILIKE $1 THEN 1 ELSE 2 END,
            d.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(&query)
    .bind(request.limit.unwrap_or(50))
    .fetch_all(pool)
    .await?;

    let results: Vec<DictionaryEntryResponse> = entries
        .into_iter()
        .map(|record| DictionaryEntryResponse {
            id: record.get("id"),
            pnar_word: record.get("pnar_word"),
            pnar_word_kbf: record.get("pnar_word_kbf"),
            english_word: record.get("english_word"),
            part_of_speech: record.get("part_of_speech"),
            definition: record.get("definition"),
            example_pnar: record.get("example_pnar"),
            example_english: record.get("example_english"),
            difficulty_level: record.get("difficulty_level"),
            usage_frequency: record.get("usage_frequency"),
            cultural_context: record.get("cultural_context"),
            related_words: record.get("related_words"),
            pronunciation: record.get("pronunciation"),
            etymology: record.get("etymology"),
            verified: record.get("verified"),
            created_at: record.get("created_at"),
            updated_at: record.get("updated_at"),
            created_by: record.get("created_by"),
            created_by_email: record.get("created_by_email"),
            updated_by: record.get("updated_by"),
            updated_by_email: record.get("updated_by_email"),
            verified_by: record.get("verified_by"),
            verified_by_email: record.get("verified_by_email"),
            verified_at: record.get("verified_at"),
        })
        .collect();

    // Track search analytics
    if let Err(e) = AnalyticsTracker::track_search(
        pool,
        &request.query,
        user_id,
        session_id,
        results.len(),
        ip_address,
        user_agent,
    ).await {
        tracing::warn!("Failed to track search analytics: {}", e);
    }

    Ok(results)
}

pub async fn update_entry(
    pool: &PgPool,
    entry_id: Uuid,
    user_id: Uuid,
    request: UpdateDictionaryEntryRequest,
) -> Result<DictionaryEntryResponse, AppError> {
    // First, check if the entry exists and user has permission
    let existing = sqlx::query("SELECT created_by FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(pool)
        .await?;

    let existing =
        existing.ok_or_else(|| AppError::NotFound(error_messages::DICTIONARY_ENTRY_NOT_FOUND))?;
    let created_by: Option<Uuid> = existing.get("created_by");

    if created_by != Some(user_id) {
        return Err(AppError::Forbidden(
            error_messages::YOU_CAN_ONLY_UPDATE_YOUR_OWN_ENTRIES,
        ));
    }

    let entry_record = sqlx::query(
        r#"
        UPDATE pnar_dictionary 
        SET 
            pnar_word = COALESCE($2, pnar_word),
            pnar_word_kbf = COALESCE($3, pnar_word_kbf),
            english_word = COALESCE($4, english_word),
            part_of_speech = COALESCE($5, part_of_speech),
            definition = COALESCE($6, definition),
            example_pnar = COALESCE($7, example_pnar),
            example_english = COALESCE($8, example_english),
            difficulty_level = COALESCE($9, difficulty_level),
            usage_frequency = COALESCE($10, usage_frequency),
            cultural_context = COALESCE($11, cultural_context),
            related_words = COALESCE($12, related_words),
            pronunciation = COALESCE($13, pronunciation),
            etymology = COALESCE($14, etymology),
            updated_by = $15,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, pnar_word, pnar_word_kbf, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by, updated_by, verified_by, verified_at
        "#,
    )
    .bind(entry_id)
    .bind(&request.pnar_word)
    .bind(&request.pnar_word_kbf)
    .bind(&request.english_word)
    .bind(&request.part_of_speech)
    .bind(&request.definition)
    .bind(&request.example_pnar)
    .bind(&request.example_english)
    .bind(request.difficulty_level)
    .bind(request.usage_frequency)
    .bind(&request.cultural_context)
    .bind(&request.related_words)
    .bind(&request.pronunciation)
    .bind(&request.etymology)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Internal(
                    "Dictionary entry with this pnar_word already exists".to_string(),
                );
            }
        }
        AppError::Database(e)
    })?;

    let response = build_dictionary_response(pool, &entry_record).await?;

    // Track contribution analytics for update
    let points = AnalyticsTracker::calculate_contribution_points("dictionary_entry", "update");
    let new_value = serde_json::json!({
        "pnar_word": request.pnar_word,
        "english_word": request.english_word,
        "part_of_speech": request.part_of_speech,
        "definition": request.definition
    });

    if let Err(e) = AnalyticsTracker::track_contribution(
        pool,
        user_id,
        "dictionary_entry",
        "pnar_dictionary",
        entry_id,
        "update",
        None, // Could fetch previous values if needed
        Some(new_value),
        points,
    ).await {
        tracing::warn!("Failed to track contribution analytics: {}", e);
    }

    Ok(response)
}

pub async fn delete_entry(pool: &PgPool, entry_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    // First, check if the entry exists and user has permission
    let existing = sqlx::query("SELECT created_by FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(pool)
        .await?;

    let existing =
        existing.ok_or_else(|| AppError::NotFound(error_messages::DICTIONARY_ENTRY_NOT_FOUND))?;
    let created_by: Option<Uuid> = existing.get("created_by");

    if created_by != Some(user_id) {
        return Err(AppError::Forbidden(
            error_messages::YOU_CAN_ONLY_DELETE_YOUR_OWN_ENTRIES,
        ));
    }
    sqlx::query("DELETE FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .execute(pool)
        .await?;

    Ok(())
}

// Admin-only function to delete any entry
pub async fn admin_delete_entry(pool: &PgPool, entry_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(error_messages::DICTIONARY_ENTRY_NOT_FOUND));
    }

    Ok(())
}

pub async fn verify_entry(
    pool: &PgPool,
    entry_id: Uuid,
    verifier_id: Uuid,
) -> Result<DictionaryEntryResponse, AppError> {
    let entry_record = sqlx::query(
        r#"
        UPDATE pnar_dictionary 
        SET verified = true, verified_by = $2, verified_at = NOW(), updated_by = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, pnar_word, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by, updated_by, verified_by, verified_at
        "#,
    )
    .bind(entry_id)
    .bind(verifier_id)
    .fetch_optional(pool)
    .await?;

    let entry_record =
        entry_record.ok_or_else(|| AppError::NotFound(error_messages::DICTIONARY_ENTRY_NOT_FOUND))?;

    let response = build_dictionary_response(pool, &entry_record).await?;

    // Track contribution analytics for verification
    let points = AnalyticsTracker::calculate_contribution_points("dictionary_entry", "verify");
    let new_value = serde_json::json!({
        "verified": true,
        "verified_by": verifier_id,
        "verified_at": chrono::Utc::now()
    });

    if let Err(e) = AnalyticsTracker::track_contribution(
        pool,
        verifier_id,
        "dictionary_entry",
        "pnar_dictionary",
        entry_id,
        "verify",
        None,
        Some(new_value),
        points,
    ).await {
        tracing::warn!("Failed to track contribution analytics: {}", e);
    }

    Ok(response)
}
