// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, Utc};
use sqlx::{MySql, Pool};

use crate::{
    error_chirho::AppErrorChirho,
    models_chirho::schedule_chirho::ScheduleChirho,
};

#[derive(serde::Deserialize)]
pub struct ScheduleQueryChirho {
    pub church_id_chirho: Option<String>,
    pub start_date_chirho: Option<NaiveDate>,
    pub end_date_chirho: Option<NaiveDate>,
}

pub async fn get_public_schedules_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    Query(query_chirho): Query<ScheduleQueryChirho>,
) -> Result<Json<Vec<ScheduleChirho>>, AppErrorChirho> {
    let schedules_chirho = if let Some(church_id_chirho) = query_chirho.church_id_chirho {
        if let (Some(start_date_chirho), Some(end_date_chirho)) = (query_chirho.start_date_chirho, query_chirho.end_date_chirho) {
            // Filter by church and date range
            sqlx::query_as!(
                ScheduleChirho,
                r#"
                SELECT 
                    schedule_id_chirho,
                    church_id_chirho,
                    worship_date_chirho,
                    assigned_by_admin_id_chirho,
                    created_timestamp_chirho
                FROM scheduled_worship_days_chirho
                WHERE church_id_chirho = ? 
                AND worship_date_chirho BETWEEN ? AND ?
                ORDER BY worship_date_chirho ASC
                "#,
                church_id_chirho,
                start_date_chirho,
                end_date_chirho
            )
            .fetch_all(&pool_chirho)
            .await?
        } else {
            // Filter by church only
            sqlx::query_as!(
                ScheduleChirho,
                r#"
                SELECT 
                    schedule_id_chirho,
                    church_id_chirho,
                    worship_date_chirho,
                    assigned_by_admin_id_chirho,
                    created_timestamp_chirho
                FROM scheduled_worship_days_chirho
                WHERE church_id_chirho = ?
                ORDER BY worship_date_chirho ASC
                "#,
                church_id_chirho
            )
            .fetch_all(&pool_chirho)
            .await?
        }
    } else if let (Some(start_date_chirho), Some(end_date_chirho)) = (query_chirho.start_date_chirho, query_chirho.end_date_chirho) {
        // Filter by date range only
        sqlx::query_as!(
            ScheduleChirho,
            r#"
            SELECT 
                schedule_id_chirho,
                church_id_chirho,
                worship_date_chirho,
                assigned_by_admin_id_chirho,
                created_timestamp_chirho
            FROM scheduled_worship_days_chirho
            WHERE worship_date_chirho BETWEEN ? AND ?
            ORDER BY worship_date_chirho ASC
            "#,
            start_date_chirho,
            end_date_chirho
        )
        .fetch_all(&pool_chirho)
        .await?
    } else {
        // No filters, get all schedules
        sqlx::query_as!(
            ScheduleChirho,
            r#"
            SELECT 
                schedule_id_chirho,
                church_id_chirho,
                worship_date_chirho,
                assigned_by_admin_id_chirho,
                created_timestamp_chirho
            FROM scheduled_worship_days_chirho
            ORDER BY worship_date_chirho ASC
            "#
        )
        .fetch_all(&pool_chirho)
        .await?
    };

    Ok(Json(schedules_chirho))
}

pub async fn get_schedule_chirho(
    State(pool_chirho): State<Pool<MySql>>,
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

pub async fn get_upcoming_schedules_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    Query(query_chirho): Query<ScheduleQueryChirho>,
) -> Result<Json<Vec<ScheduleChirho>>, AppErrorChirho> {
    let today_chirho = Utc::now().date_naive();
    
    let schedules_chirho = if let Some(church_id_chirho) = query_chirho.church_id_chirho {
        // Filter by church and upcoming dates
        sqlx::query_as!(
            ScheduleChirho,
            r#"
            SELECT 
                schedule_id_chirho,
                church_id_chirho,
                worship_date_chirho,
                assigned_by_admin_id_chirho,
                created_timestamp_chirho
            FROM scheduled_worship_days_chirho
            WHERE church_id_chirho = ? 
            AND worship_date_chirho >= ?
            ORDER BY worship_date_chirho ASC
            "#,
            church_id_chirho,
            today_chirho
        )
        .fetch_all(&pool_chirho)
        .await?
    } else {
        // Get all upcoming schedules
        sqlx::query_as!(
            ScheduleChirho,
            r#"
            SELECT 
                schedule_id_chirho,
                church_id_chirho,
                worship_date_chirho,
                assigned_by_admin_id_chirho,
                created_timestamp_chirho
            FROM scheduled_worship_days_chirho
            WHERE worship_date_chirho >= ?
            ORDER BY worship_date_chirho ASC
            "#,
            today_chirho
        )
        .fetch_all(&pool_chirho)
        .await?
    };

    Ok(Json(schedules_chirho))
} 