// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChurchChirho {
    pub church_id_chirho: String,
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChurchChirho {
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChurchChirho {
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
} 