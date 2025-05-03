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
        let worship_start_hour_chirho = calculate_worship_start_hour_chirho(church_chirho.church_timezone_chirho.clone(), continent_timezone_chirho);
        

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChurchWithContinentResponseChirho {
    pub church_id_chirho: String,
    pub name_chirho: String,
    pub continent_id_chirho: Option<String>,
    pub continent_name_chirho: Option<String>,
    pub continent_timezone_chirho: Option<String>,
    pub church_timezone_chirho: String,   
    pub worship_start_hour_chirho: Option<i32>,
    pub leader_name_chirho: Option<String>,
    pub leader_email_chirho: Option<String>,
    pub admin_details_note_chirho: Option<String>,
    pub internal_notes_chirho: Option<String>,
    pub member_access_token_chirho: String,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

fn calculate_worship_start_hour_chirho(church_timezone_chirho: String, continent_timezone_chirho: Option<String>) -> i32 {
    if continent_timezone_chirho.is_none() {
        return 18;
    }

    let continent_tz_chirho = continent_timezone_chirho.unwrap();
    let now_chirho = chrono::Utc::now();
    let continent_time_chirho = now_chirho.with_timezone(&chrono_tz::Tz::from_str(&continent_tz_chirho).unwrap());
    let church_time_chirho = now_chirho.with_timezone(&chrono_tz::Tz::from_str(&church_timezone_chirho).unwrap());

    let hour_diff_chirho = church_time_chirho.hour() as i32 - continent_time_chirho.hour() as i32;
    18 + hour_diff_chirho
}

impl ChurchWithContinentResponseChirho {

    pub fn fill_worship_start_hour_chirho(&mut self) -> () {
        let continent_timezone_chirho = self.continent_timezone_chirho.clone();
        let worship_start_hour_chirho = calculate_worship_start_hour_chirho(self.church_timezone_chirho.clone(), continent_timezone_chirho);
        self.worship_start_hour_chirho = Some(worship_start_hour_chirho);
    }

    pub fn from_church_chirho(church_chirho: ChurchChirho, continent_name_chirho: Option<String>) -> Self {

        let self_chirho = Self {
            church_id_chirho: church_chirho.church_id_chirho,
            name_chirho: church_chirho.name_chirho,
            continent_id_chirho: church_chirho.continent_id_chirho,
            continent_name_chirho,
            worship_start_hour_chirho: None,
            continent_timezone_chirho: None,
            church_timezone_chirho: church_chirho.church_timezone_chirho,
            leader_name_chirho: church_chirho.leader_name_chirho,
            leader_email_chirho: church_chirho.leader_email_chirho,
            admin_details_note_chirho: church_chirho.admin_details_note_chirho,
            internal_notes_chirho: church_chirho.internal_notes_chirho,
            member_access_token_chirho: church_chirho.member_access_token_chirho,
            created_timestamp_chirho: church_chirho.created_timestamp_chirho,
            updated_timestamp_chirho: church_chirho.updated_timestamp_chirho,
        };

        self_chirho
    }
} 