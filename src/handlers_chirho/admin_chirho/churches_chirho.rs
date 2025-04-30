// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use crate::{
    error_chirho::AppErrorChirho,
    middleware_chirho::AuthStateChirho,
    models_chirho::church_chirho::{ChurchChirho, CreateChurchChirho, UpdateChurchChirho},
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use rand::{Rng, rngs::OsRng};
use sqlx::{MySql, Pool};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_church_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Json(church_chirho): Json<CreateChurchChirho>,
) -> Result<Json<ChurchChirho>, AppErrorChirho> {
    let church_id_chirho = Uuid::new_v4().to_string();
    let now_chirho = Utc::now();
    
    // Generate a random access token using OsRng
    let token_chirho: String = (0..32)
        .map(|_| OsRng.sample(rand::distributions::Alphanumeric) as char)
        .collect();

    // First insert the church
    sqlx::query!(
        r#"
        INSERT INTO churches_chirho (
            church_id_chirho,
            name_chirho,
            continent_id_chirho,
            church_timezone_chirho,
            admin_details_note_chirho,
            internal_notes_chirho,
            member_access_token_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        church_id_chirho,
        church_chirho.name_chirho,
        church_chirho.continent_id_chirho,
        church_chirho.church_timezone_chirho,
        church_chirho.admin_details_note_chirho,
        church_chirho.internal_notes_chirho,
        token_chirho,
        now_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Then fetch the created church
    let created_church_chirho = sqlx::query_as!(
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
        WHERE church_id_chirho = ?
        "#,
        church_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(created_church_chirho))
}

pub async fn get_churches_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
) -> Result<Json<Vec<ChurchChirho>>, AppErrorChirho> {
    let churches_chirho = sqlx::query_as!(
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
        ORDER BY name_chirho
        "#
    )
    .fetch_all(&pool_chirho)
    .await?;

    Ok(Json(churches_chirho))
}

pub async fn get_church_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(church_id_chirho): Path<String>,
) -> Result<Json<ChurchChirho>, AppErrorChirho> {
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
        WHERE church_id_chirho = ?
        "#,
        church_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?;

    match church_chirho {
        Some(church_chirho) => Ok(Json(church_chirho)),
        None => Err(AppErrorChirho::NotFound("Church not found".to_string())),
    }
}

pub async fn update_church_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(church_id_chirho): Path<String>,
    Json(update_chirho): Json<UpdateChurchChirho>,
) -> Result<Json<ChurchChirho>, AppErrorChirho> {
    let now_chirho = Utc::now();

    // First update the church
    let result_chirho = sqlx::query!(
        r#"
        UPDATE churches_chirho
        SET 
            name_chirho = ?,
            continent_id_chirho = ?,
            church_timezone_chirho = ?,
            admin_details_note_chirho = ?,
            internal_notes_chirho = ?,
            updated_timestamp_chirho = ?
        WHERE church_id_chirho = ?
        "#,
        update_chirho.name_chirho,
        update_chirho.continent_id_chirho,
        update_chirho.church_timezone_chirho,
        update_chirho.admin_details_note_chirho,
        update_chirho.internal_notes_chirho,
        now_chirho,
        church_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Church not found".to_string()));
    }

    // Then fetch the updated church
    let updated_church_chirho = sqlx::query_as!(
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
        WHERE church_id_chirho = ?
        "#,
        church_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(updated_church_chirho))
}

pub async fn delete_church_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(church_id_chirho): Path<String>,
) -> Result<(), AppErrorChirho> {
    let result_chirho = sqlx::query!(
        r#"
        DELETE FROM churches_chirho
        WHERE church_id_chirho = ?
        "#,
        church_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    if result_chirho.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Church not found".to_string()));
    }

    Ok(())
} 