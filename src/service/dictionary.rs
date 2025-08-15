use crate::domain::dictionary::{DictionaryEntry, DictionaryResponse};
use crate::model::request::dictionary::{CreateDictionaryRequest, UpdateDictionaryRequest, SearchDictionaryRequest};
use crate::model::response::ErrorCode;
use sqlx::PgPool;

#[doc = "Create a new dictionary entry"]
pub async fn create_dictionary_entry(
    data: CreateDictionaryRequest,
    created_by: uuid::Uuid,
    pool: &PgPool,
) -> Result<DictionaryResponse, ErrorCode> {
    // Check if the Pnar word already exists
    if let Ok(Some(_)) = DictionaryEntry::find_by_pnar_word(&data.pnar_word, pool).await {
        return Err(ErrorCode::INTERNAL001); // You might want to create a specific error for duplicate entries
    }

    let entry = DictionaryEntry {
        id: uuid::Uuid::new_v4(), // This will be overridden by the database
        pnar_word: data.pnar_word,
        english_word: data.english_word,
        part_of_speech: data.part_of_speech,
        definition: data.definition,
        example_pnar: data.example_pnar,
        example_english: data.example_english,
        difficulty_level: data.difficulty_level,
        usage_frequency: data.usage_frequency,
        cultural_context: data.cultural_context,
        related_words: data.related_words,
        pronunciation: data.pronunciation,
        etymology: data.etymology,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by: Some(created_by),
        verified: false,
        verified_by: None,
        verified_at: None,
    };

    let created_entry = DictionaryEntry::create(&entry, pool).await?;
    Ok(DictionaryResponse::from(created_entry))
}

#[doc = "Get dictionary entry by ID"]
pub async fn get_dictionary_entry(
    id: uuid::Uuid,
    pool: &PgPool,
) -> Result<Option<DictionaryResponse>, ErrorCode> {
    let entry = DictionaryEntry::find_by_id(id, pool).await?;
    Ok(entry.map(DictionaryResponse::from))
}

#[doc = "Get all dictionary entries with pagination"]
pub async fn get_all_dictionary_entries(
    limit: Option<i64>,
    offset: Option<i64>,
    pool: &PgPool,
) -> Result<Vec<DictionaryResponse>, ErrorCode> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    
    let entries = DictionaryEntry::find_all(limit, offset, pool).await?;
    Ok(entries.into_iter().map(DictionaryResponse::from).collect())
}

#[doc = "Search dictionary entries"]
pub async fn search_dictionary_entries(
    search_request: SearchDictionaryRequest,
    pool: &PgPool,
) -> Result<Vec<DictionaryResponse>, ErrorCode> {
    let search_type = search_request.search_type.as_deref().unwrap_or("english");
    
    let entries = match search_type {
        "pnar" => {
            if let Ok(Some(entry)) = DictionaryEntry::find_by_pnar_word(&search_request.query, pool).await {
                vec![entry]
            } else {
                vec![]
            }
        },
        "english" | _ => {
            DictionaryEntry::search_by_english_word(&search_request.query, pool).await?
        }
    };

    Ok(entries.into_iter().map(DictionaryResponse::from).collect())
}

#[doc = "Update dictionary entry"]
pub async fn update_dictionary_entry(
    id: uuid::Uuid,
    data: UpdateDictionaryRequest,
    _updated_by: uuid::Uuid,
    pool: &PgPool,
) -> Result<DictionaryResponse, ErrorCode> {
    let mut entry = DictionaryEntry::find_by_id(id, pool).await?
        .ok_or(ErrorCode::INTERNAL001)?; // Entry not found

    // Update only the fields that are provided
    if let Some(pnar_word) = data.pnar_word {
        entry.pnar_word = pnar_word;
    }
    if let Some(english_word) = data.english_word {
        entry.english_word = english_word;
    }
    if let Some(part_of_speech) = data.part_of_speech {
        entry.part_of_speech = Some(part_of_speech);
    }
    if let Some(definition) = data.definition {
        entry.definition = Some(definition);
    }
    if let Some(example_pnar) = data.example_pnar {
        entry.example_pnar = Some(example_pnar);
    }
    if let Some(example_english) = data.example_english {
        entry.example_english = Some(example_english);
    }
    if let Some(difficulty_level) = data.difficulty_level {
        entry.difficulty_level = Some(difficulty_level);
    }
    if let Some(usage_frequency) = data.usage_frequency {
        entry.usage_frequency = Some(usage_frequency);
    }
    if let Some(cultural_context) = data.cultural_context {
        entry.cultural_context = Some(cultural_context);
    }
    if let Some(related_words) = data.related_words {
        entry.related_words = Some(related_words);
    }
    if let Some(pronunciation) = data.pronunciation {
        entry.pronunciation = Some(pronunciation);
    }
    if let Some(etymology) = data.etymology {
        entry.etymology = Some(etymology);
    }

    let updated_entry = entry.update(pool).await?;
    Ok(DictionaryResponse::from(updated_entry))
}

#[doc = "Delete dictionary entry"]
pub async fn delete_dictionary_entry(
    id: uuid::Uuid,
    pool: &PgPool,
) -> Result<bool, ErrorCode> {
    DictionaryEntry::delete(id, pool).await
}

#[doc = "Verify dictionary entry"]
pub async fn verify_dictionary_entry(
    id: uuid::Uuid,
    verified_by: uuid::Uuid,
    pool: &PgPool,
) -> Result<DictionaryResponse, ErrorCode> {
    let mut entry = DictionaryEntry::find_by_id(id, pool).await?
        .ok_or(ErrorCode::INTERNAL001)?; // Entry not found

    entry.verified = true;
    entry.verified_by = Some(verified_by);
    entry.verified_at = Some(chrono::Utc::now());

    let updated_entry = entry.update(pool).await?;
    Ok(DictionaryResponse::from(updated_entry))
}
