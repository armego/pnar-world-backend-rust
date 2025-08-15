use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub id: Uuid,
    pub word: String,
    pub definition: String,
    pub examples: Vec<String>,
    pub tags: Vec<String>,
    pub context: Option<String>,
    pub author_id: Uuid,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
