use crate::{
    dto::{PnarAlphabetResponse, CreatePnarAlphabetRequest, UpdatePnarAlphabetRequest},
    error::AppError,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Get all alphabet characters ordered by sort_order
pub async fn list_alphabets(pool: &PgPool) -> Result<Vec<PnarAlphabetResponse>, AppError> {
    let records = sqlx::query(
        r#"
        SELECT id, small, capital, kbf_small, kbf_capital, sort_order, created_at
        FROM pnar_alphabets
        ORDER BY sort_order ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let alphabets: Vec<PnarAlphabetResponse> = records
        .into_iter()
        .map(|record| PnarAlphabetResponse {
            id: record.get("id"),
            small: record.get("small"),
            capital: record.get("capital"),
            kbf_small: record.get("kbf_small"),
            kbf_capital: record.get("kbf_capital"),
            sort_order: record.get("sort_order"),
            created_at: record.get("created_at"),
        })
        .collect();

    Ok(alphabets)
}

/// Get a specific alphabet character by ID
pub async fn get_alphabet(pool: &PgPool, alphabet_id: Uuid) -> Result<PnarAlphabetResponse, AppError> {
    let record = sqlx::query(
        r#"
        SELECT id, small, capital, kbf_small, kbf_capital, sort_order, created_at
        FROM pnar_alphabets
        WHERE id = $1
        "#
    )
    .bind(alphabet_id)
    .fetch_optional(pool)
    .await?;

    match record {
        Some(record) => Ok(PnarAlphabetResponse {
            id: record.get("id"),
            small: record.get("small"),
            capital: record.get("capital"),
            kbf_small: record.get("kbf_small"),
            kbf_capital: record.get("kbf_capital"),
            sort_order: record.get("sort_order"),
            created_at: record.get("created_at"),
        }),
        None => Err(AppError::NotFound("Alphabet character not found".to_string())),
    }
}

/// Create a new alphabet character mapping
pub async fn create_alphabet(
    pool: &PgPool,
    request: CreatePnarAlphabetRequest,
) -> Result<PnarAlphabetResponse, AppError> {
    let alphabet_id = Uuid::new_v4();

    // Check if character already exists
    let existing = sqlx::query("SELECT id FROM pnar_alphabets WHERE small = $1")
        .bind(&request.small)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict(format!(
            "Alphabet character '{}' already exists",
            request.small
        )));
    }

    let record = sqlx::query(
        r#"
        INSERT INTO pnar_alphabets (id, small, capital, kbf_small, kbf_capital, sort_order, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, small, capital, kbf_small, kbf_capital, sort_order, created_at
        "#
    )
    .bind(alphabet_id)
    .bind(&request.small)
    .bind(&request.capital)
    .bind(&request.kbf_small)
    .bind(&request.kbf_capital)
    .bind(request.sort_order)
    .fetch_one(pool)
    .await?;

    Ok(PnarAlphabetResponse {
        id: record.get("id"),
        small: record.get("small"),
        capital: record.get("capital"),
        kbf_small: record.get("kbf_small"),
        kbf_capital: record.get("kbf_capital"),
        sort_order: record.get("sort_order"),
        created_at: record.get("created_at"),
    })
}

/// Update an alphabet character mapping
pub async fn update_alphabet(
    pool: &PgPool,
    alphabet_id: Uuid,
    request: UpdatePnarAlphabetRequest,
) -> Result<PnarAlphabetResponse, AppError> {
    let record = sqlx::query(
        r#"
        UPDATE pnar_alphabets
        SET 
            small = COALESCE($2, small),
            capital = COALESCE($3, capital),
            kbf_small = COALESCE($4, kbf_small),
            kbf_capital = COALESCE($5, kbf_capital),
            sort_order = COALESCE($6, sort_order)
        WHERE id = $1
        RETURNING id, small, capital, kbf_small, kbf_capital, sort_order, created_at
        "#
    )
    .bind(alphabet_id)
    .bind(&request.small)
    .bind(&request.capital)
    .bind(&request.kbf_small)
    .bind(&request.kbf_capital)
    .bind(request.sort_order)
    .fetch_optional(pool)
    .await?;

    match record {
        Some(record) => Ok(PnarAlphabetResponse {
            id: record.get("id"),
            small: record.get("small"),
            capital: record.get("capital"),
            kbf_small: record.get("kbf_small"),
            kbf_capital: record.get("kbf_capital"),
            sort_order: record.get("sort_order"),
            created_at: record.get("created_at"),
        }),
        None => Err(AppError::NotFound("Alphabet character not found".to_string())),
    }
}

/// Delete an alphabet character mapping
pub async fn delete_alphabet(pool: &PgPool, alphabet_id: Uuid) -> Result<(), AppError> {
    let rows_affected = sqlx::query("DELETE FROM pnar_alphabets WHERE id = $1")
        .bind(alphabet_id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound("Alphabet character not found".to_string()));
    }

    Ok(())
}

/// Convert text from traditional Pnar to keyboard-friendly format
pub async fn convert_to_kbf(pool: &PgPool, text: &str) -> Result<String, AppError> {
    let alphabets = list_alphabets(pool).await?;
    let mut converted = text.to_string();

    // Replace special characters with keyboard-friendly equivalents
    for alphabet in alphabets {
        converted = converted.replace(&alphabet.small, &alphabet.kbf_small);
        converted = converted.replace(&alphabet.capital, &alphabet.kbf_capital);
    }

    Ok(converted)
}

/// Convert text from keyboard-friendly to traditional Pnar format
pub async fn convert_from_kbf(pool: &PgPool, text: &str) -> Result<String, AppError> {
    let alphabets = list_alphabets(pool).await?;
    let mut converted = text.to_string();

    // Sort by kbf length (descending) to handle multi-character mappings first
    let mut sorted_alphabets = alphabets;
    sorted_alphabets.sort_by(|a, b| b.kbf_small.len().cmp(&a.kbf_small.len()));

    // Replace keyboard-friendly equivalents with special characters
    for alphabet in sorted_alphabets {
        converted = converted.replace(&alphabet.kbf_small, &alphabet.small);
        converted = converted.replace(&alphabet.kbf_capital, &alphabet.capital);
    }

    Ok(converted)
}
