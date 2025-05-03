// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use crate::{
    error_chirho::AppErrorChirho,
    models_chirho::church_chirho::ChurchChirho,
};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySql, Pool};

pub async fn get_church_by_token_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    Path(token_chirho): Path<String>,
) -> Result<Json<ChurchChirho>, AppErrorChirho> {
    println!("get_church_by_token_chirho: token_chirho: {}", token_chirho);
    let church_chirho = sqlx::query_as!(
        ChurchChirho,
        r#"
        SELECT 
            church_id_chirho,
            name_chirho,
            continent_id_chirho,
            church_timezone_chirho,
            leader_name_chirho,
            leader_email_chirho,
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
    .await?;

    match church_chirho {
        Some(church_chirho) => Ok(Json(church_chirho)),
        None => Err(AppErrorChirho::NotFound("Church not found".to_string())),
    }
} 