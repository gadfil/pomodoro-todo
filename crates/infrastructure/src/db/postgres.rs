use std::time::Duration;
use sqlx::{PgPool, postgres::PgPoolOptions};
use shared::config::{DatabaseConfig, };

pub async fn create_pool(database_config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(database_config.max_connections)
        .max_connections(database_config.min_connections)
        .acquire_timeout(Duration::from_secs(database_config.acquire_timeout_secs))
        .connect(&database_config.url)
    .await
}