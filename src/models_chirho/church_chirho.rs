// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChurchChirho {
    pub church_id_chirho: String,
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub leader_name_chirho: Option<String>,
    pub leader_email_chirho: Option<String>,
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
    pub leader_name_chirho: Option<String>,
    pub leader_email_chirho: Option<String>,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChurchChirho {
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub leader_name_chirho: Option<String>,
    pub leader_email_chirho: Option<String>,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
}

#[derive(Debug, Serialize)]
pub struct ChurchResponseChirho {
    pub church_id_chirho: String,
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub church_timezone_chirho: String,
    pub worship_start_hour_chirho: i32,
    pub leader_name_chirho: Option<String>,
    pub leader_email_chirho: Option<String>,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

impl ChurchResponseChirho {
    pub fn from_church_chirho(church_chirho: ChurchChirho, continent_timezone_chirho: Option<String>) -> Self {
        // Calculate the start hour based on timezone difference
        let worship_start_hour_chirho = if let Some(continent_tz_chirho) = continent_timezone_chirho {
            // Get the current time in both timezones
            let now_chirho = chrono::Utc::now();
            let continent_time_chirho = now_chirho.with_timezone(&chrono_tz::Tz::from_str(&continent_tz_chirho).unwrap());
            let church_time_chirho = now_chirho.with_timezone(&chrono_tz::Tz::from_str(&church_chirho.church_timezone_chirho).unwrap());

            // Calculate the hour difference
            let hour_diff_chirho = church_time_chirho.hour() as i32 - continent_time_chirho.hour() as i32;

            println!("Hallelujah, the hour difference is: {}", hour_diff_chirho);
            
            // Adjust the start hour based on the timezone difference
            18 + hour_diff_chirho
        } else {
            println!("Hallelujah, no continent timezone found, defaulting to 6 PM");
            18 // Default to 6 PM if no continent timezone
        };

        Self {
            church_id_chirho: church_chirho.church_id_chirho,
            name_chirho: church_chirho.name_chirho,
            continent_id_chirho: church_chirho.continent_id_chirho,
            church_timezone_chirho: church_chirho.church_timezone_chirho,
            worship_start_hour_chirho,
            leader_name_chirho: church_chirho.leader_name_chirho,
            leader_email_chirho: church_chirho.leader_email_chirho,
            admin_details_note_chirho: church_chirho.admin_details_note_chirho,
            internal_notes_chirho: church_chirho.internal_notes_chirho,
            member_access_token_chirho: church_chirho.member_access_token_chirho,
            created_timestamp_chirho: church_chirho.created_timestamp_chirho,
            updated_timestamp_chirho: church_chirho.updated_timestamp_chirho,
        }
    }
} 