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

impl<S> FromRequestParts<S> for AuthStateChirho
where
    S: Send + Sync,
{
    type Rejection = AppErrorChirho;

    async fn from_request_parts(parts_chirho: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        #[cfg(feature = "logging_chirho")]
        println!("Auth middleware: Starting token extraction");
        
        // First try to get token from Authorization header
        let token_chirho = parts_chirho
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header_chirho| header_chirho.to_str().ok())
            .and_then(|header_chirho| header_chirho.strip_prefix("Bearer "))
            .map(|token_chirho| token_chirho.to_string());

        #[cfg(feature = "logging_chirho")]
        println!("Auth middleware: Authorization header token: {:?}", token_chirho);

        // If no token in header, try cookie
        let token_chirho = if let Some(token_chirho) = token_chirho {
            token_chirho
        } else {
            #[cfg(feature = "logging_chirho")]
            println!("Auth middleware: No token in header, trying cookie");
            let jar_chirho = parts_chirho.extract::<CookieJar>().await.unwrap();
            jar_chirho
                .get("auth_token_chirho")
                .map(|cookie_chirho| cookie_chirho.value().to_string())
                .ok_or_else(|| AppErrorChirho::Authentication("Missing auth token".to_string()))?
        };

        #[cfg(feature = "logging_chirho")]
        println!("Auth middleware: Using token: {}", token_chirho);

        let config_chirho = crate::config_chirho::ConfigChirho::from_env();

        #[cfg(feature = "logging_chirho")]
        {
            println!("Auth middleware: Config found, secret length: {}", config_chirho.jwt_chirho.secret_chirho.len());
            println!("Auth middleware: Creating decoding key...");
        }

        let decoding_key_chirho = DecodingKey::from_secret(config_chirho.jwt_chirho.secret_chirho.as_bytes());

        #[cfg(feature = "logging_chirho")]
        println!("Auth middleware: Decoding key created, attempting to decode token...");

        let mut validation_chirho = Validation::default();
        validation_chirho.validate_exp = true;
        validation_chirho.required_spec_claims = std::collections::HashSet::from(["exp_chirho".to_string()]);

        let token_data_chirho = decode::<ClaimsChirho>(
            &token_chirho,
            &decoding_key_chirho,
            &validation_chirho,
        )
        .map_err(|e| {

            #[cfg(feature = "logging_chirho")]
            {
                println!("Auth middleware: Token decode error: {:?}", e);
                println!("Auth middleware: Error kind: {:?}", e.kind());
            }

            AppErrorChirho::Authentication(format!("Invalid token: {:?}", e))
        })?;

        #[cfg(feature = "logging_chirho")]
        {
            println!("Auth middleware: Token decoded successfully");
            println!("Auth middleware: Claims: {:?}", token_data_chirho.claims);
        }
        Ok(AuthStateChirho {
            claims_chirho: token_data_chirho.claims,
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