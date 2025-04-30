// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use crate::{
    error_chirho::AppErrorChirho,
    middleware_chirho::AuthStateChirho,
    models_chirho::continent_chirho::{ContinentChirho, CreateContinentChirho, UpdateContinentChirho},
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use sqlx::{MySql, Pool};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_continent_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Json(continent_chirho): Json<CreateContinentChirho>,
) -> Result<Json<ContinentChirho>, AppErrorChirho> {
    let continent_id_chirho = Uuid::new_v4().to_string();
    let now_chirho = Utc::now();

    // First insert the continent
    sqlx::query!(
        r#"
        INSERT INTO continents_chirho (
            continent_id_chirho,
            name_chirho,
            central_timezone_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        )
        VALUES (?, ?, ?, ?, ?)
        "#,
        continent_id_chirho,
        continent_chirho.name_chirho,
        continent_chirho.central_timezone_chirho,
        now_chirho,
        now_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Then fetch the created continent
    let created_continent_chirho = sqlx::query_as!(
        ContinentChirho,
        r#"
        SELECT 
            continent_id_chirho,
            name_chirho,
            central_timezone_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM continents_chirho
        WHERE continent_id_chirho = ?
        "#,
        continent_id_chirho
    )
    .fetch_one(&pool_chirho)
    .await?;

    Ok(Json(created_continent_chirho))
}

pub async fn get_continents_chirho(
    State(pool_chirho): State<Pool<MySql>>,
    auth_chirho: AuthStateChirho,
) -> Result<Json<Vec<ContinentChirho>>, AppErrorChirho> {
    println!("Continents handler: Starting with auth claims: {:?}", auth_chirho.claims_chirho);
    
    let continents_chirho = sqlx::query_as!(
        ContinentChirho,
        r#"
        SELECT 
            continent_id_chirho,
            name_chirho,
            central_timezone_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM continents_chirho
        ORDER BY name_chirho
        "#
    )
    .fetch_all(&pool_chirho)
    .await
    .map_err(|e| {
        println!("Continents handler: Database error: {:?}", e);
        AppErrorChirho::Database(e)
    })?;

    println!("Continents handler: Successfully retrieved {} continents", continents_chirho.len());
    Ok(Json(continents_chirho))
}

pub async fn get_continent_chirho(
    State(pool): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(continent_id): Path<String>,
) -> Result<Json<ContinentChirho>, AppErrorChirho> {
    let continent = sqlx::query_as!(
        ContinentChirho,
        r#"
        SELECT 
            continent_id_chirho,
            name_chirho,
            central_timezone_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM continents_chirho
        WHERE continent_id_chirho = ?
        "#,
        continent_id
    )
    .fetch_optional(&pool)
    .await?;

    match continent {
        Some(continent) => Ok(Json(continent)),
        None => Err(AppErrorChirho::NotFound("Continent not found".to_string())),
    }
}

pub async fn update_continent_chirho(
    State(pool): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(continent_id): Path<String>,
    Json(continent): Json<UpdateContinentChirho>,
) -> Result<Json<ContinentChirho>, AppErrorChirho> {
    let now = Utc::now();

    // First update the continent
    let result = sqlx::query!(
        r#"
        UPDATE continents_chirho
        SET 
            name_chirho = ?,
            central_timezone_chirho = ?,
            updated_timestamp_chirho = ?
        WHERE continent_id_chirho = ?
        "#,
        continent.name_chirho,
        continent.central_timezone_chirho,
        now,
        continent_id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Continent not found".to_string()));
    }

    // Then fetch the updated continent
    let updated_continent = sqlx::query_as!(
        ContinentChirho,
        r#"
        SELECT 
            continent_id_chirho,
            name_chirho,
            central_timezone_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM continents_chirho
        WHERE continent_id_chirho = ?
        "#,
        continent_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated_continent))
}

pub async fn delete_continent_chirho(
    State(pool): State<Pool<MySql>>,
    _auth_chirho: AuthStateChirho,
    Path(continent_id): Path<String>,
) -> Result<(), AppErrorChirho> {
    let result = sqlx::query!(
        r#"
        DELETE FROM continents_chirho
        WHERE continent_id_chirho = ?
        "#,
        continent_id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppErrorChirho::NotFound("Continent not found".to_string()));
    }

    Ok(())
} 