use std::sync::Arc;
use crate::{error_chirho::AppErrorChirho, middleware_chirho::AuthStateChirho, models_chirho::schedule_chirho::{CreateScheduleChirho, ScheduleChirho, UpdateScheduleChirho}, AppStateChirho};
// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use sqlx::{MySql, MySqlPool, Pool};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub async fn get_schedules_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    _auth_chirho: AuthStateChirho,
) -> Result<Json<Vec<ScheduleChirho>>, AppErrorChirho> {
    let schedules_chirho = sqlx::query_as!(
        ScheduleChirho,
        r#"
        SELECT 
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        ORDER BY worship_date_chirho DESC
        "#
    )
    .fetch_all(&pool_chirho)
    .await?;

    Ok(Json(schedules_chirho))
}

pub async fn get_schedule_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(schedule_id_chirho): Path<String>,
) -> Result<Json<ScheduleChirho>, AppErrorChirho> {
    let schedule_chirho = sqlx::query_as!(
        ScheduleChirho,
        r#"
        SELECT 
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ?
        "#,
        schedule_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?;

    match schedule_chirho {
        Some(schedule_chirho) => Ok(Json(schedule_chirho)),
        None => Err(AppErrorChirho::NotFound("Schedule not found".to_string())),
    }
}

pub async fn create_schedule_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    _auth_chirho: AuthStateChirho,
    Json(schedule_chirho): Json<CreateScheduleChirho>,
) -> Result<Json<ScheduleChirho>, AppErrorChirho> {
    println!("HALLELUJAH CREATE SCHEDULE");
    let schedule_id_chirho = Uuid::new_v4().to_string();
    let now_chirho = Utc::now();

    // First insert the schedule
    sqlx::query!(
        r#"
        INSERT INTO scheduled_worship_days_chirho (
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?)
        "#,
        schedule_id_chirho,
        schedule_chirho.church_id_chirho,
        schedule_chirho.worship_date_chirho,
        schedule_chirho.assigned_by_admin_id_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Then fetch the created schedule
    let created_schedule_chirho = sqlx::query_as!(
        ScheduleChirho,
        r#"
        SELECT 
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ?
        "#,
        schedule_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(created_schedule_chirho))
}

pub async fn update_schedule_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(schedule_id_chirho): Path<String>,
    Json(update_chirho): Json<UpdateScheduleChirho>,
) -> Result<Json<ScheduleChirho>, AppErrorChirho> {
    let _now_chirho = Utc::now();

    // First update the schedule
    let result_chirho = sqlx::query!(
        r#"
        UPDATE scheduled_worship_days_chirho
        SET 
            church_id_chirho = ?,
            worship_date_chirho = ?,
            assigned_by_admin_id_chirho = ?
        WHERE schedule_id_chirho = ?
        "#,
        update_chirho.church_id_chirho,
        update_chirho.worship_date_chirho,
        update_chirho.assigned_by_admin_id_chirho,
        schedule_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Schedule not found".to_string()));
    }

    // Then fetch the updated schedule
    let updated_schedule_chirho = sqlx::query_as!(
        ScheduleChirho,
        r#"
        SELECT 
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ?
        "#,
        schedule_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(updated_schedule_chirho))
}

pub async fn delete_schedule_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    _auth_chirho: AuthStateChirho,
    Path(schedule_id_chirho): Path<String>,
) -> Result<(), AppErrorChirho> {
    let result_chirho = sqlx::query!(
        r#"
        DELETE FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ?
        "#,
        schedule_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Schedule not found".to_string()));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignScheduleRequestChirho {
    pub church_id_chirho: String,
    pub worship_date_chirho: chrono::NaiveDate,
}

pub async fn assign_schedule_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    auth_chirho: AuthStateChirho,
    Json(assignment_chirho): Json<AssignScheduleRequestChirho>,
) -> Result<Json<ScheduleChirho>, AppErrorChirho> {
    let schedule_id_chirho = Uuid::new_v4().to_string();
    let now_chirho = Utc::now();

    // First insert the schedule
    sqlx::query!(
        r#"
        INSERT INTO scheduled_worship_days_chirho (
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?)
        "#,
        schedule_id_chirho,
        assignment_chirho.church_id_chirho,
        assignment_chirho.worship_date_chirho,
        auth_chirho.claims_chirho.sub_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Then fetch the created schedule
    let created_schedule_chirho = sqlx::query_as!(
        ScheduleChirho,
        r#"
        SELECT 
            schedule_id_chirho,
            church_id_chirho,
            worship_date_chirho,
            assigned_by_admin_id_chirho,
            created_timestamp_chirho
        FROM scheduled_worship_days_chirho
        WHERE schedule_id_chirho = ?
        "#,
        schedule_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(created_schedule_chirho))
}

pub async fn unassign_schedule_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path((church_id_chirho, worship_date_chirho)): Path<(String, chrono::NaiveDate)>,
) -> Result<(), AppErrorChirho> {
    let result_chirho = sqlx::query!(
        r#"
        DELETE FROM scheduled_worship_days_chirho
        WHERE church_id_chirho = ? AND worship_date_chirho = ?
        "#,
        church_id_chirho,
        worship_date_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Schedule not found".to_string()));
    }

    Ok(())
} 