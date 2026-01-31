use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::dto::auth::{RegisterRequest, RegisterResponse};
use crate::errors::AuthError;
use crate::repositories::{user, verification};
use crate::utils::generate_code;

pub async fn register(pool: &PgPool, req: RegisterRequest) -> Result<RegisterResponse, AuthError> {
    if let Some(existing) = user::find_by_email(pool, &req.email).await? {
        if existing.email_verified{
            return Err(AuthError::EmailTaken);
        }else {
            return Err(AuthError::EmailNotConfirmed);
        }
    }

    let password_hash = hash_password(&req.password)?;
    let display_name = req.name.unwrap_or_else(|| req.email.clone());

    let mut tx = pool.begin().await?;

    let created_user = user::create(&mut *tx, &req.email, &password_hash, &display_name).await?;

    let code = generate_code();
    let expires_at = Utc::now() + Duration::minutes(15);
    verification::create_email_confirm(&mut *tx, created_user.id, &code, expires_at).await?;

    tx.commit().await?;

    Ok(RegisterResponse {
        id: created_user.id,
        email: created_user.email.unwrap_or_default(),
        message: "User registered successfully".to_string(),
    })
}

fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::PasswordHash(e.to_string()))?;
    Ok(hash.to_string())
}
