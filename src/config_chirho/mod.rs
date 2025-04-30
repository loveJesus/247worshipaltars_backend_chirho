// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use axum::extract::FromRef;
use serde::Deserialize;
use sqlx::{MySql, Pool};
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigChirho {
    pub database_chirho: DatabaseConfigChirho,
    pub server_chirho: ServerConfigChirho,
    pub jwt_chirho: JwtConfigChirho,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfigChirho {
    pub url_chirho: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfigChirho {
    pub host_chirho: String,
    pub port_chirho: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfigChirho {
    pub secret_chirho: String,
    pub expiration_chirho: i64,
}

impl ConfigChirho {
    pub fn from_env() -> Self {
        ConfigChirho {
            database_chirho: DatabaseConfigChirho {
                url_chirho: env::var("DATABASE_URL_CHIRHO")
                    .unwrap_or_else(|_| "mysql://root:password@localhost:3306/worship_coordinator_chirho".to_string()),
            },
            server_chirho: ServerConfigChirho {
                host_chirho: env::var("HOST_CHIRHO").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port_chirho: env::var("PORT_CHIRHO")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .expect("PORT_CHIRHO must be a number"),
            },
            jwt_chirho: JwtConfigChirho {
                secret_chirho: env::var("JWT_SECRET_CHIRHO")
                    .expect("JWT_SECRET_CHIRHO must be set"),
                expiration_chirho: env::var("JWT_EXPIRATION_CHIRHO")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .expect("JWT_EXPIRATION_CHIRHO must be a number"),
            },
        }
    }
}

impl FromRef<Pool<MySql>> for ConfigChirho {
    fn from_ref(_: &Pool<MySql>) -> Self {
        Self::from_env()
    }
} 