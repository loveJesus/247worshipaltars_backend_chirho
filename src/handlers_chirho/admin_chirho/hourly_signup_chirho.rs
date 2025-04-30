use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{
    error_chirho::AppErrorChirho,
    middleware_chirho::AuthStateChirho,
    models_chirho::hourly_signup_chirho::{CreateHourlySignupChirho, HourlySignupChirho, UpdateHourlySignupChirho},
};

// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

pub async fn get_hourly_signups_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
) -> Result<Json<Vec<HourlySignupChirho>>, AppErrorChirho> {
    let signups_chirho = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        SELECT 
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM hourly_signups_chirho
        ORDER BY schedule_id_chirho, slot_hour_chirho
        "#
    )
    .fetch_all(&pool_chirho)
    .await?;

    Ok(Json(signups_chirho))
}

pub async fn get_hourly_signup_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(signup_id_chirho): Path<String>,
) -> Result<Json<HourlySignupChirho>, AppErrorChirho> {
    let signup_chirho = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        SELECT 
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM hourly_signups_chirho
        WHERE signup_id_chirho = ?
        "#,
        signup_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?;

    match signup_chirho {
        Some(signup_chirho) => Ok(Json(signup_chirho)),
        None => Err(AppErrorChirho::NotFound("Hourly signup not found".to_string())),
    }
}

pub async fn create_hourly_signup_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Json(signup_chirho): Json<CreateHourlySignupChirho>,
) -> Result<Json<HourlySignupChirho>, AppErrorChirho> {
    let signup_id_chirho = Uuid::new_v4().to_string();
    let now_chirho = Utc::now();

    // First insert the signup
    sqlx::query!(
        r#"
        INSERT INTO hourly_signups_chirho (
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        signup_id_chirho,
        signup_chirho.schedule_id_chirho,
        signup_chirho.slot_hour_chirho,
        signup_chirho.participant_name_chirho,
        now_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Then fetch the created signup
    let created_signup_chirho = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        SELECT 
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM hourly_signups_chirho
        WHERE signup_id_chirho = ?
        "#,
        signup_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(created_signup_chirho))
}

pub async fn update_hourly_signup_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(signup_id_chirho): Path<String>,
    Json(update_chirho): Json<UpdateHourlySignupChirho>,
) -> Result<Json<HourlySignupChirho>, AppErrorChirho> {
    let now_chirho = Utc::now();

    // First update the signup
    let result_chirho = sqlx::query!(
        r#"
        UPDATE hourly_signups_chirho
        SET 
            schedule_id_chirho = ?,
            slot_hour_chirho = ?,
            participant_name_chirho = ?,
            updated_timestamp_chirho = ?
        WHERE signup_id_chirho = ?
        "#,
        update_chirho.schedule_id_chirho,
        update_chirho.slot_hour_chirho,
        update_chirho.participant_name_chirho,
        now_chirho,
        signup_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Hourly signup not found".to_string()));
    }

    // Then fetch the updated signup
    let updated_signup_chirho = sqlx::query_as!(
        HourlySignupChirho,
        r#"
        SELECT 
            signup_id_chirho,
            schedule_id_chirho,
            slot_hour_chirho,
            participant_name_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM hourly_signups_chirho
        WHERE signup_id_chirho = ?
        "#,
        signup_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(updated_signup_chirho))
}

pub async fn delete_hourly_signup_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(signup_id_chirho): Path<String>,
) -> Result<(), AppErrorChirho> {
    let result_chirho = sqlx::query!(
        r#"
        DELETE FROM hourly_signups_chirho
        WHERE signup_id_chirho = ?
        "#,
        signup_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Hourly signup not found".to_string()));
    }

    Ok(())
} 