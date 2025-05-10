use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::fs;

use crate::{
    error_chirho::AppErrorChirho,
    AppStateChirho,
};

pub async fn serve_pdf_file_chirho(
    State((pool_chirho, _)): State<(MySqlPool, Arc<AppStateChirho>)>,
    Path(pdf_file_id_chirho): Path<String>,
) -> Result<impl IntoResponse, AppErrorChirho> {
    // Get file path and check if it's public
    let file_info = sqlx::query!(
        r#"
        SELECT file_path_chirho, file_name_chirho, is_public_chirho
        FROM pdf_files_chirho
        WHERE pdf_file_id_chirho = ?
        "#,
        pdf_file_id_chirho
    )
    .fetch_optional(&pool_chirho)
    .await?
    .ok_or_else(|| AppErrorChirho::NotFound("PDF file not found".to_string()))?;

    if !file_info.is_public_chirho.map(|v| v != 0).unwrap_or(false) {
        return Err(AppErrorChirho::Forbidden("This file is not public".to_string()));
    }

    // Read file content
    let content = fs::read(&file_info.file_path_chirho)
        .await
        .map_err(|e| AppErrorChirho::InternalError(format!("Failed to read file: {}", e)))?;

    // Set headers for PDF download
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/pdf".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("inline; filename=\"{}\"", file_info.file_name_chirho).parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, content))
} 