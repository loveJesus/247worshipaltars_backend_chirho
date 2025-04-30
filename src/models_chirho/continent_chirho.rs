// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContinentChirho {
    pub continent_id_chirho: String,
    pub name_chirho: String,
    pub central_timezone_chirho: String,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContinentChirho {
    pub name_chirho: String,
    pub central_timezone_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContinentChirho {
    pub name_chirho: String,
    pub central_timezone_chirho: String,
} 