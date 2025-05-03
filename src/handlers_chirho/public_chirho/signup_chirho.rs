use crate::{
    error_chirho::AppErrorChirho,
    models_chirho::{
        church_chirho::ChurchChirho,
        hourly_signup_chirho::{CreateHourlySignupChirho, HourlySignupChirho},
        scheduled_worship_day_chirho::ScheduledWorshipDayChirho,
    },
};
// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use uuid;
use chrono;

pub async fn create_signup_chirho(
    State(pool_chirho): State<MySqlPool>,
    Path((church_token_chirho, schedule_id_chirho)): Path<(String, String)>,
    Json(signup_chirho): Json<CreateHourlySignupChirho>,
) -> Result<Json<HourlySignupChirho>, AppErrorChirho> {
    println!("HALLELUJAH {:?}", signup_chirho);
    // First verify the church exists and get its ID
    let church_chirho = sqlx::query_as!(
        ChurchChirho,
        r#"
        SELECT
            church_id_chirho,
            name_chirho,
            continent_id_chirho,
            church_timezone_chirho,
            admin_details_note_chirho,
            leader_name_chirho,
            leader_email_chirho,
            internal_notes_chirho,
            member_access_token_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM churches_chirho
        WHERE member_access_token_chirho = ?
        "#,
        church_token_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?
    .ok_or_else(|| AppErrorChirho::NotFound("Church not found".to_string()))?;

    // Verify the schedule exists and belongs to this church
    let _schedule_chirho = sqlx::query_as!(
        ScheduledWorshipDayChirho,
        r#"
        SELECT
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ? AND church_id_chirho = ?
        "#,
        schedule_id_chirho,
        church_chirho.church_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?
    .ok_or_else(|| AppErrorChirho::NotFound("Schedule not found".to_string()))?;

    // Create the signup
    let signup_id_chirho = uuid::Uuid::new_v4().to_string();
    let now_chirho = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO hourly_signups_chirho (
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            signup_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?)
        "#,
        signup_id_chirho,
        schedule_id_chirho,
        signup_chirho.slot_hour_chirho,
        signup_chirho.participant_name_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Fetch the created signup
    let hourly_signup_chirho = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        SELECT 
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            signup_timestamp_chirho
        FROM hourly_signups_chirho
        WHERE signup_id_chirho = ?
        "#,
        signup_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(hourly_signup_chirho))
} 