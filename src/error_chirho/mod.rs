// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::BcryptError;
use serde::Serialize;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrorChirho {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Auth error: {0}")]
    AuthError(String),
}

impl IntoResponse for AppErrorChirho {
    fn into_response(self) -> Response {
        let status = match self {
            AppErrorChirho::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorChirho::Validation(_) => StatusCode::BAD_REQUEST,
            AppErrorChirho::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppErrorChirho::NotFound(_) => StatusCode::NOT_FOUND,
            AppErrorChirho::Bcrypt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorChirho::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorChirho::AuthError(_) => StatusCode::UNAUTHORIZED,
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
        }

        let body = ErrorResponse {
            error: self.to_string(),
        };

        (status, axum::Json(body)).into_response()
    }
} 