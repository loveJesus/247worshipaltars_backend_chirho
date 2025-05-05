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
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppErrorChirho {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppErrorChirho::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppErrorChirho::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppErrorChirho::Authentication(_) => (StatusCode::UNAUTHORIZED, "Authentication error"),
            AppErrorChirho::JwtError(_) => (StatusCode::UNAUTHORIZED, "JWT error"),
            AppErrorChirho::Bcrypt(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Bcrypt error"),
            AppErrorChirho::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppErrorChirho::NotFound(_) => (StatusCode::NOT_FOUND, "Not found"),
            AppErrorChirho::Forbidden(_) => (StatusCode::FORBIDDEN, "Forbidden"),
            AppErrorChirho::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
        };

        let body = format!("{}: {}", error_message, self.to_string());

        (status, body).into_response()
    }
} 