use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Database connection failed: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Email already taken")]
    EmailTaken,
    #[error("Email address is not confirmed")]
    EmailNotConfirmed,

    #[error("Password hashing failed: {0}")]
    PasswordHash(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
