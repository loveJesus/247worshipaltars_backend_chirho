// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use std::sync::Arc;
use crate::{error_chirho::AppErrorChirho, models_chirho::church_chirho::{ChurchChirho, ChurchResponseChirho, ChurchWithContinentResponseChirho}, AppStateChirho};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySql, MySqlPool, Pool};

pub async fn get_church_by_token_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    Path(church_token_chirho): Path<String>,
) -> Result<Json<ChurchWithContinentResponseChirho>, AppErrorChirho> {
    println!("get_church_by_token_chirho: token_chirho: {}", church_token_chirho);
    
    // First get the church with continent name
    let mut church_response_chirho = sqlx::query_as!(
        ChurchWithContinentResponseChirho,
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
            c_chirho.updated_timestamp_chirho,
            CAST(NULL AS INTEGER) as worship_start_hour_chirho,
            cont_chirho.name_chirho as continent_name_chirho,
            cont_chirho.central_timezone_chirho as continent_timezone_chirho
        FROM churches_chirho c_chirho
        LEFT JOIN continents_chirho cont_chirho ON c_chirho.continent_id_chirho = cont_chirho.continent_id_chirho
        WHERE c_chirho.member_access_token_chirho = ?
        "#,
        church_token_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?;

    match church_response_chirho {
        Some(mut inner_church_chirho) => {
            inner_church_chirho.fill_worship_start_hour_chirho();
            Ok(Json(inner_church_chirho))
        }
        None => Err(AppErrorChirho::NotFound("Church not found".to_string())),
    }
} 