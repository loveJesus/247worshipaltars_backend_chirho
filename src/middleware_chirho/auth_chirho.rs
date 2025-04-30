// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use crate::config_chirho::ConfigChirho;
use crate::error_chirho::AppErrorChirho;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsChirho {
    pub sub_chirho: String,
    pub exp_chirho: usize,
    pub role_chirho: String,
}

pub struct AuthStateChirho {
    pub claims_chirho: ClaimsChirho,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthStateChirho
where
    S: Send + Sync,
{
    type Rejection = AppErrorChirho;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jar = parts.extract::<CookieJar>().await.unwrap();
        let token = jar
            .get("auth_token_chirho")
            .ok_or_else(|| AppErrorChirho::AuthError("Missing auth token".to_string()))?
            .value();

        let config = parts
            .extensions
            .get::<ConfigChirho>()
            .ok_or_else(|| AppErrorChirho::InternalError("Config not found".to_string()))?;

        let token_data = decode::<ClaimsChirho>(
            token,
            &DecodingKey::from_secret(config.jwt_chirho.secret_chirho.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppErrorChirho::AuthError("Invalid token".to_string()))?;

        Ok(AuthStateChirho {
            claims_chirho: token_data.claims,
        })
    }
}

pub fn create_token_chirho(
    subject: String,
    role: String,
    config: &ConfigChirho,
) -> Result<String, AppErrorChirho> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let exp = now + config.jwt_chirho.expiration_chirho as usize;

    let claims = ClaimsChirho {
        sub_chirho: subject,
        exp_chirho: exp,
        role_chirho: role,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_chirho.secret_chirho.as_bytes()),
    )
    .map_err(|_| AppErrorChirho::InternalError("Failed to create token".to_string()))
} 