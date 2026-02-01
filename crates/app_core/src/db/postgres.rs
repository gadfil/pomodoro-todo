use crate::config::DatabaseConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(database_config.max_connections)
        .min_connections(database_config.min_connections)
        .acquire_timeout(Duration::from_secs(database_config.acquire_timeout_secs))
        .connect(&database_config.url)
        .await
}
