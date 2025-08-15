use crate::{
    dto::{
        responses::{DictionaryEntryResponse, DictionaryPaginatedResponse},
        CreateDictionaryEntryRequest, SearchDictionaryRequest, UpdateDictionaryEntryRequest,
    },
    error::AppError,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

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
        return Err(AppError::Conflict(format!(
            "Pnar word '{}' already exists",
            request.pnar_word
        )));
    }

    let entry_record = sqlx::query(
        r#"
        INSERT INTO pnar_dictionary (
            id, pnar_word, english_word, part_of_speech, definition,
            example_pnar, example_english, difficulty_level, usage_frequency,
            cultural_context, related_words, pronunciation, etymology,
            created_by, created_at, updated_at, verified
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW(), NOW(), $15
        )
        RETURNING id, pnar_word, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by
        "#
    )
    .bind(entry_id)
    .bind(&request.pnar_word)
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
                return AppError::Conflict(format!("Dictionary entry with pnar_word '{}' already exists", request.pnar_word));
            }
        }
        AppError::Database(e)
    })?;

    Ok(DictionaryEntryResponse {
        id: entry_record.get("id"),
        pnar_word: entry_record.get("pnar_word"),
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
        created_by: entry_record.get("created_by"),
    })
}

pub async fn get_entry(pool: &PgPool, entry_id: Uuid) -> Result<DictionaryEntryResponse, AppError> {
    let entry_record = sqlx::query(
        r#"
        SELECT id, pnar_word, english_word, part_of_speech, definition,
               example_pnar, example_english, difficulty_level, usage_frequency,
               cultural_context, related_words, pronunciation, etymology,
               verified, created_at, updated_at, created_by
        FROM pnar_dictionary 
        WHERE id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(pool)
    .await?;

    let entry_record =
        entry_record.ok_or_else(|| AppError::NotFound("Dictionary entry not found".to_string()))?;

    Ok(DictionaryEntryResponse {
        id: entry_record.get("id"),
        pnar_word: entry_record.get("pnar_word"),
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
        created_by: entry_record.get("created_by"),
    })
}

pub async fn list_entries(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<DictionaryPaginatedResponse, AppError> {
    let offset = (page - 1) * per_page;

    let entries = sqlx::query(
        r#"
        SELECT id, pnar_word, english_word, part_of_speech, definition,
               example_pnar, example_english, difficulty_level, usage_frequency,
               cultural_context, related_words, pronunciation, etymology,
               verified, created_at, updated_at, created_by
        FROM pnar_dictionary 
        ORDER BY created_at DESC
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
        })
        .collect();

    Ok(DictionaryPaginatedResponse::new(
        items, page, per_page, total,
    ))
}

pub async fn search_entries(
    pool: &PgPool,
    request: SearchDictionaryRequest,
) -> Result<Vec<DictionaryEntryResponse>, AppError> {
    let query = format!("%{}%", request.query);

    let entries = sqlx::query(
        r#"
        SELECT id, pnar_word, english_word, part_of_speech, definition,
               example_pnar, example_english, difficulty_level, usage_frequency,
               cultural_context, related_words, pronunciation, etymology,
               verified, created_at, updated_at, created_by
        FROM pnar_dictionary 
        WHERE pnar_word ILIKE $1 OR english_word ILIKE $1 OR definition ILIKE $1
        ORDER BY 
            CASE WHEN pnar_word ILIKE $1 THEN 1 ELSE 2 END,
            created_at DESC
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
        })
        .collect();

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
        existing.ok_or_else(|| AppError::NotFound("Dictionary entry not found".to_string()))?;
    let created_by: Option<Uuid> = existing.get("created_by");

    if created_by != Some(user_id) {
        return Err(AppError::Forbidden(
            "You can only update your own entries".to_string(),
        ));
    }

    let entry_record = sqlx::query(
        r#"
        UPDATE pnar_dictionary 
        SET 
            pnar_word = COALESCE($2, pnar_word),
            english_word = COALESCE($3, english_word),
            part_of_speech = COALESCE($4, part_of_speech),
            definition = COALESCE($5, definition),
            example_pnar = COALESCE($6, example_pnar),
            example_english = COALESCE($7, example_english),
            difficulty_level = COALESCE($8, difficulty_level),
            usage_frequency = COALESCE($9, usage_frequency),
            cultural_context = COALESCE($10, cultural_context),
            related_words = COALESCE($11, related_words),
            pronunciation = COALESCE($12, pronunciation),
            etymology = COALESCE($13, etymology),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, pnar_word, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by
        "#,
    )
    .bind(entry_id)
    .bind(&request.pnar_word)
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
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict(
                    "Dictionary entry with this pnar_word already exists".to_string(),
                );
            }
        }
        AppError::Database(e)
    })?;

    Ok(DictionaryEntryResponse {
        id: entry_record.get("id"),
        pnar_word: entry_record.get("pnar_word"),
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
        created_by: entry_record.get("created_by"),
    })
}

pub async fn delete_entry(pool: &PgPool, entry_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    // First, check if the entry exists and user has permission
    let existing = sqlx::query("SELECT created_by FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(pool)
        .await?;

    let existing =
        existing.ok_or_else(|| AppError::NotFound("Dictionary entry not found".to_string()))?;
    let created_by: Option<Uuid> = existing.get("created_by");

    if created_by != Some(user_id) {
        return Err(AppError::Forbidden(
            "You can only delete your own entries".to_string(),
        ));
    }

    sqlx::query("DELETE FROM pnar_dictionary WHERE id = $1")
        .bind(entry_id)
        .execute(pool)
        .await?;

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
        SET verified = true, verified_by = $2, verified_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING id, pnar_word, english_word, part_of_speech, definition,
                  example_pnar, example_english, difficulty_level, usage_frequency,
                  cultural_context, related_words, pronunciation, etymology,
                  verified, created_at, updated_at, created_by
        "#,
    )
    .bind(entry_id)
    .bind(verifier_id)
    .fetch_optional(pool)
    .await?;

    let entry_record =
        entry_record.ok_or_else(|| AppError::NotFound("Dictionary entry not found".to_string()))?;

    Ok(DictionaryEntryResponse {
        id: entry_record.get("id"),
        pnar_word: entry_record.get("pnar_word"),
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
        created_by: entry_record.get("created_by"),
    })
}
