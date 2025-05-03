// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use crate::{
    error_chirho::AppErrorChirho,
    models_chirho::church_chirho::{ChurchChirho, ChurchResponseChirho},
};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySql, Pool};

pub async fn get_church_by_token_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    Path(church_token_chirho): Path<String>,
) -> Result<Json<ChurchResponseChirho>, AppErrorChirho> {
    println!("get_church_by_token_chirho: token_chirho: {}", church_token_chirho);
    
    // First get the church
    let church_chirho = sqlx::query_as!(
        ChurchChirho,
        r#"
        SELECT 
            c_chirho.church_id_chirho,
            c_chirho.name_chirho,
            c_chirho.continent_id_chirho,
            c_chirho.church_timezone_chirho,
            c_chirho.leader_name_chirho,
            c_chirho.leader_email_chirho,
            c_chirho.admin_details_note_chirho,
            c_chirho.internal_notes_chirho,
            c_chirho.member_access_token_chirho,
            c_chirho.created_timestamp_chirho,
            c_chirho.updated_timestamp_chirho
        FROM churches_chirho c_chirho
        WHERE c_chirho.member_access_token_chirho = ?
        "#,
        church_token_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?;

    // Then get the continent timezone if the church has a continent
    let continent_timezone_chirho = if let Some(church_chirho) = &church_chirho {
        if let Some(continent_id_chirho) = &church_chirho.continent_id_chirho {
            sqlx::query!(
                r#"
                SELECT central_timezone_chirho
                FROM continents_chirho
                WHERE continent_id_chirho = ?
                "#,
                continent_id_chirho
            )
            .fetch_optional(&pool_chirho)
            .await?
            .map(|row| row.central_timezone_chirho)
        } else {
            None
        }
    } else {
        None
    };

    match church_chirho {
        Some(church_chirho) => Ok(Json(ChurchResponseChirho::from_church_chirho(church_chirho, continent_timezone_chirho))),
        None => Err(AppErrorChirho::NotFound("Church not found".to_string())),
    }
} 