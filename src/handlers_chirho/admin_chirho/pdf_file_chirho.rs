use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    error_chirho::AppErrorChirho,
    models_chirho::pdf_file_chirho::{CreatePdfFileChirho, PdfFileChirho, PdfFileResponseChirho},
    AppStateChirho,
};

pub async fn upload_pdf_file_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    mut multipart: Multipart,
) -> Result<Json<PdfFileResponseChirho>, AppErrorChirho> {
    let mut file_name_chirho = None;
    let mut description_chirho = None;
    let mut is_public_chirho = false;
    let mut file_content = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| AppErrorChirho::BadRequest(format!("Failed to process multipart field: {}", e)))? {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                let file_data = field.bytes().await.map_err(|e| AppErrorChirho::BadRequest(format!("Failed to read file data: {}", e)))?;
                file_content = Some(file_data);
            }
            "file_name" => {
                file_name_chirho = Some(field.text().await.map_err(|e| AppErrorChirho::BadRequest(format!("Failed to read file name: {}", e)))?);
            }
            "description" => {
                description_chirho = Some(field.text().await.map_err(|e| AppErrorChirho::BadRequest(format!("Failed to read description: {}", e)))?);
            }
            "is_public" => {
                is_public_chirho = field.text().await.map_err(|e| AppErrorChirho::BadRequest(format!("Failed to read is_public: {}", e)))?.to_lowercase() == "true";
            }
            _ => {}
        }
    }

    let file_content = file_content.ok_or_else(|| AppErrorChirho::BadRequest("No file provided".to_string()))?;
    let file_name_chirho = file_name_chirho.ok_or_else(|| AppErrorChirho::BadRequest("No file name provided".to_string()))?;

    // Create uploads directory if it doesn't exist
    let uploads_dir = "uploads_chirho";
    if !std::path::Path::new(uploads_dir).exists() {
        std::fs::create_dir_all(uploads_dir).map_err(|e| {
            eprintln!("Failed to create uploads directory: {}", e);
            AppErrorChirho::InternalError(format!("Failed to create uploads directory: {}", e))
        })?;
    }

    // Generate unique filename
    let file_extension = file_name_chirho.split('.').last().unwrap_or("pdf");
    let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
    let file_path = format!("{}/{}", uploads_dir, unique_filename);

    // Save file
    tokio::fs::write(&file_path, &file_content).await.map_err(|e| {
        eprintln!("Failed to write file: {}", e);
        AppErrorChirho::InternalError(format!("Failed to write file: {}", e))
    })?;

    // Create database record
    let create_pdf_file_chirho = CreatePdfFileChirho {
        church_id_chirho: "global".to_string(), // Using "global" for admin-uploaded files
        file_name_chirho,
        description_chirho,
        is_public_chirho,
    };

    let pdf_file_chirho = PdfFileChirho::new(create_pdf_file_chirho, file_path);

    sqlx::query!(
        r#"
        INSERT INTO pdf_files_chirho (
            pdf_file_id_chirho,
            church_id_chirho,
            file_name_chirho,
            file_path_chirho,
            description_chirho,
            is_public_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        pdf_file_chirho.pdf_file_id_chirho,
        pdf_file_chirho.church_id_chirho,
        pdf_file_chirho.file_name_chirho,
        pdf_file_chirho.file_path_chirho,
        pdf_file_chirho.description_chirho,
        pdf_file_chirho.is_public_chirho,
        pdf_file_chirho.created_timestamp_chirho,
        pdf_file_chirho.updated_timestamp_chirho
    )
    .execute(&pool_chirho)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        AppErrorChirho::Database(e)
    })?;

    Ok(Json(PdfFileResponseChirho::from(pdf_file_chirho)))
}

pub async fn get_all_pdf_files_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
) -> Result<Json<Vec<PdfFileResponseChirho>>, AppErrorChirho> {
    let pdf_files = sqlx::query!(
        r#"
        SELECT 
            pdf_file_id_chirho,
            church_id_chirho,
            file_name_chirho,
            file_path_chirho,
            description_chirho,
            is_public_chirho,
            created_timestamp_chirho,
            updated_timestamp_chirho
        FROM pdf_files_chirho
        "#
    )
    .fetch_all(&pool_chirho)
    .await?
    .into_iter()
    .map(|row| PdfFileChirho {
        pdf_file_id_chirho: row.pdf_file_id_chirho,
        church_id_chirho: row.church_id_chirho,
        file_name_chirho: row.file_name_chirho,
        file_path_chirho: row.file_path_chirho,
        description_chirho: row.description_chirho,
        is_public_chirho: row.is_public_chirho.map(|v| v != 0).unwrap_or(false),
        created_timestamp_chirho: row.created_timestamp_chirho.unwrap_or_else(|| Utc::now()),
        updated_timestamp_chirho: row.updated_timestamp_chirho.unwrap_or_else(|| Utc::now()),
    })
    .collect::<Vec<_>>();

    Ok(Json(pdf_files.into_iter().map(PdfFileResponseChirho::from).collect()))
}

pub async fn delete_pdf_file_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    Path(pdf_file_id_chirho): Path<String>,
) -> Result<StatusCode, AppErrorChirho> {
    // Get file path before deleting
    let file_path = sqlx::query!(
        r#"
        SELECT file_path_chirho
        FROM pdf_files_chirho
        WHERE pdf_file_id_chirho = ?
        "#,
        pdf_file_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?
    .ok_or_else(|| AppErrorChirho::NotFound("PDF file not found".to_string()))?
    .file_path_chirho;

    // Delete from database
    sqlx::query!(
        r#"
        DELETE FROM pdf_files_chirho
        WHERE pdf_file_id_chirho = ?
        "#,
        pdf_file_id_chirho
    )
    .execute(&pool_chirho)
    .await?;

    // Delete file from filesystem
    if let Err(e) = fs::remove_file(&file_path).await {
        eprintln!("Error deleting file {}: {}", file_path, e);
    }

    Ok(StatusCode::NO_CONTENT)
} 