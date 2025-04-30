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

pub async fn create_signup_chirho(
    State(pool_chirho): State<MySqlPool>,
    Path(token_chirho): Path<String>,
    Json(signup_chirho): Json<CreateHourlySignupChirho>,
) -> Result<Json<HourlySignupChirho>, AppErrorChirho> {
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
            internal_notes_chirho,
            member_access_token_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM churches_chirho
        WHERE member_access_token_chirho = ?
        "#,
        token_chirho
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
        signup_chirho.schedule_id_chirho,
        church_chirho.church_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?
    .ok_or_else(|| AppErrorChirho::NotFound("Schedule not found".to_string()))?;

    // Create the signup
    let mysql_row_signup_chirho  = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        INSERT INTO hourly_signups_chirho (
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho
        )
        VALUES (?, ?, ?)
        RETURNING
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            signup_timestamp_chirho
        "#,
        signup_chirho.schedule_id_chirho,
        signup_chirho.slot_hour_chirho,
        signup_chirho.participant_name_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;
    // convert mysql row to HourlySignupChirho
    let hourly_signup_chirho  = HourlySignupChirho {
        signup_id_chirho: mysql_row_signup_chirho.try_get("signup_id_chirho")?,
        schedule_id_chirho: mysql_row_signup_chirho.try_get("schedule_id_chirho")?,
        slot_hour_chirho: mysql_row_signup_chirho.try_get("slot_hour_chirho")?,
        participant_name_chirho: mysql_row_signup_chirho.try_get("participant_name_chirho")?,
        signup_timestamp_chirho: mysql_row_signup_chirho.try_get("signup_timestamp_chirho")?,
    };

    Ok(Json(hourly_signup_chirho))
} 