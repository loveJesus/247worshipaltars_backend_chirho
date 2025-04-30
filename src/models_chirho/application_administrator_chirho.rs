// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApplicationAdministratorChirho {
    pub admin_id_chirho: String,
    pub username_chirho: String,
    pub password_representation_chirho: String,
    pub created_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdministratorChirho {
    pub username_chirho: String,
    pub password_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestChirho {
    pub username_chirho: String,
    pub password_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseChirho {
    pub token_chirho: String,
} 