use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Request to create a new dictionary entry
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDictionaryEntryRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Pnar word must be between 1 and 255 characters"
    ))]
    #[schema(example = "ka")]
    pub pnar_word: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "English word must be between 1 and 255 characters"
    ))]
    #[schema(example = "go")]
    pub english_word: String,

    // Optional fields (all have DEFAULT or are nullable in DB)
    #[validate(length(max = 50, message = "Part of speech must be less than 50 characters"))]
    #[schema(example = "verb")]
    pub part_of_speech: Option<String>,

    #[schema(example = "To move from one place to another")]
    pub definition: Option<String>,
    #[schema(example = "Nga ka noh")]
    pub example_pnar: Option<String>,
    #[schema(example = "I go home")]
    pub example_english: Option<String>,

    #[validate(range(
        min = 1,
        max = 10,
        message = "Difficulty level must be between 1 and 10"
    ))]
    #[schema(example = 1)]
    pub difficulty_level: Option<i32>,

    #[validate(range(min = 0, message = "Usage frequency must be non-negative"))]
    #[schema(example = 10)]
    pub usage_frequency: Option<i32>,

    #[schema(example = "Common daily usage")]
    pub cultural_context: Option<String>,
    pub related_words: Option<String>,
    #[schema(example = "ka")]
    pub pronunciation: Option<String>,
    #[schema(example = "From Proto-Austroasiatic")]
    pub etymology: Option<String>,
}

/// Request to update a dictionary entry
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDictionaryEntryRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Pnar word must be between 1 and 255 characters"
    ))]
    #[schema(example = "ka")]
    pub pnar_word: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "English word must be between 1 and 255 characters"
    ))]
    #[schema(example = "go")]
    pub english_word: Option<String>,

    #[validate(length(max = 50, message = "Part of speech must be less than 50 characters"))]
    #[schema(example = "verb")]
    pub part_of_speech: Option<String>,

    #[schema(example = "Updated definition")]
    pub definition: Option<String>,
    #[schema(example = "Updated Pnar example")]
    pub example_pnar: Option<String>,
    #[schema(example = "Updated English example")]
    pub example_english: Option<String>,

    #[validate(range(
        min = 1,
        max = 10,
        message = "Difficulty level must be between 1 and 10"
    ))]
    pub difficulty_level: Option<i32>,

    #[validate(range(min = 0, message = "Usage frequency must be non-negative"))]
    pub usage_frequency: Option<i32>,

    pub cultural_context: Option<String>,
    pub related_words: Option<String>,
    pub pronunciation: Option<String>,
    pub etymology: Option<String>,
}

/// Dictionary search request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SearchDictionaryRequest {
    #[validate(length(min = 1, message = "Search query cannot be empty"))]
    #[schema(example = "go")]
    pub query: String,

    pub search_type: Option<SearchType>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    #[schema(example = 10)]
    pub limit: Option<i64>,

    #[validate(range(min = 0, message = "Offset must be non-negative"))]
    #[schema(example = 0)]
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Pnar,
    English,
    Definition,
    All,
}
