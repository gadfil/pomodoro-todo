
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Database connection failed: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}