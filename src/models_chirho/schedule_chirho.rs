// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ScheduleChirho {
    pub schedule_id_chirho: String,
    pub church_id_chirho: String,
    pub worship_date_chirho: NaiveDate,
    pub assigned_by_admin_id_chirho: Option<String>,
    pub created_timestamp_chirho: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduleChirho {
    pub church_id_chirho: String,
    pub worship_date_chirho: NaiveDate,
    pub assigned_by_admin_id_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateScheduleChirho {
    pub church_id_chirho: String,
    pub worship_date_chirho: DateTime<Utc>,
    pub assigned_by_admin_id_chirho: String,
} 