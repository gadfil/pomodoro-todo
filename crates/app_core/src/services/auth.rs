use crate::dto::auth::{RegisterRequest, RegisterResponse};
use crate::errors::AuthError;
use crate::repositories::{user, verification};
use crate::services::notification::CodeSender;
use crate::utils::generate_code;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use sqlx::PgPool;

pub async fn register(
    pool: &PgPool,
    req: RegisterRequest,
    sender: &dyn CodeSender,
) -> Result<RegisterResponse, AuthError> {
    if let Some(existing) = user::find_by_email(pool, &req.email).await? {
        if existing.email_verified {
            return Err(AuthError::EmailTaken);
        } else {
            let code = generate_code();
            let expires_at = Utc::now() + Duration::minutes(15);
            verification::create_email_confirm(pool, existing.id, &code, expires_at).await?;
            sender
                .send_code(&req.email, &code)
                .await
                .map_err(|e| AuthError::NotificationError(e.to_string()))?;
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

    sender
        .send_code(&req.email, &code)
        .await
        .map_err(|e| AuthError::NotificationError(e.to_string()))?;

    Ok(RegisterResponse {
        id: created_user.id,
        email: created_user.email.unwrap_or_default(),
        message: "User registered successfully".to_string(),
    })
}

pub async fn confirm_email(pool: &PgPool, email: String, code: String) -> Result<(), AuthError> {
    let mut tx = pool.begin().await?;
    let user = user::find_by_email(&mut *tx, &email)
        .await?
        .ok_or(AuthError::UserNotFound)?;
    let active_verification = verification::find_active_email_verification(&mut *tx, user.id)
        .await?
        .ok_or(AuthError::InvalidCode)?;

    if active_verification.expires_at < Utc::now() {
        return Err(AuthError::CodeExpired);
    }
    // todo move max_attempts to settings and migrate table
    if active_verification.attempts > 3 {
        return Err(AuthError::TooManyAttempts);
    }
    if active_verification.code != Option::from(code) {
        verification::increment_attempts(&mut *tx, active_verification.id).await?;
        return Err(AuthError::InvalidCode);
    }
    user::set_email_verified(&mut *tx, user.id).await?;
    verification::mark_as_used(&mut *tx, active_verification.id).await?;
    tx.commit().await?;

    Ok(())
}
fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::PasswordHash(e.to_string()))?;
    Ok(hash.to_string())
}
