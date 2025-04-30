// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ScheduledWorshipDayChirho {
    pub schedule_id_chirho: String,
    pub church_id_chirho: String,
    pub worship_date_chirho: chrono::NaiveDate,
    pub assigned_by_admin_id_chirho: Option<String>,
    pub created_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduledWorshipDayChirho {
    pub church_id_chirho: String,
    pub worship_date_chirho: chrono::NaiveDate,
    pub assigned_by_admin_id_chirho: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateScheduledWorshipDayChirho {
    pub church_id_chirho: String,
    pub worship_date_chirho: chrono::NaiveDate,
    pub assigned_by_admin_id_chirho: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleQueryChirho {
    pub year_chirho: i32,
    pub month_chirho: u32,
} 