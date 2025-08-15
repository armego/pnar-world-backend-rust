use serde_json::Value;

#[doc = "Request model for creating a new dictionary entry"]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateDictionaryRequest {
    pub pnar_word: String,
    pub english_word: String,
    pub part_of_speech: Option<String>,
    pub definition: Option<String>,
    pub example_pnar: Option<String>,
    pub example_english: Option<String>,
    pub difficulty_level: Option<i32>,
    pub usage_frequency: Option<i32>,
    pub cultural_context: Option<String>,
    pub related_words: Option<Value>,
    pub pronunciation: Option<String>,
    pub etymology: Option<String>,
}

#[doc = "Request model for updating a dictionary entry"]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateDictionaryRequest {
    pub pnar_word: Option<String>,
    pub english_word: Option<String>,
    pub part_of_speech: Option<String>,
    pub definition: Option<String>,
    pub example_pnar: Option<String>,
    pub example_english: Option<String>,
    pub difficulty_level: Option<i32>,
    pub usage_frequency: Option<i32>,
    pub cultural_context: Option<String>,
    pub related_words: Option<Value>,
    pub pronunciation: Option<String>,
    pub etymology: Option<String>,
}

#[doc = "Request model for searching dictionary entries"]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SearchDictionaryRequest {
    pub query: String,
    pub search_type: Option<String>, // "pnar", "english", "definition"
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
