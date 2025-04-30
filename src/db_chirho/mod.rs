use crate::config_chirho::ConfigChirho;
// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub async fn init_db_chirho(config: &ConfigChirho) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_chirho.url_chirho)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
} 