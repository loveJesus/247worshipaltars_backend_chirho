// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct HourlySignupChirho {
    pub signup_id_chirho: String,
    pub schedule_id_chirho: String,
    pub slot_hour_chirho: i32,
    pub participant_name_chirho: String,
    pub signup_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHourlySignupChirho {
    pub schedule_id_chirho: String,
    pub slot_hour_chirho: i32,
    pub participant_name_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHourlySignupChirho {
    pub slot_hour_chirho: i32,
    pub participant_name_chirho: String,
} 