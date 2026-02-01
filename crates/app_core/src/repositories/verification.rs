use crate::enums::VerificationType;
use crate::models::verification::Verification;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

pub async fn create_email_confirm<'e, E>(
    executor: E,
    user_id: Uuid,
    code: &str,
    expires_at: DateTime<Utc>,
) -> Result<Verification, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Verification>(
        "INSERT INTO verifications (user_id, type, code, expires_at)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(user_id)
    .bind(VerificationType::EmailConfirm)
    .bind(code)
    .bind(expires_at)
    .fetch_one(executor)
    .await
}

pub async fn find_active_email_verification<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<Option<Verification>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Verification>(
        "SELECT * FROM verifications WHERE type=$1 AND user_id = $2 AND used IS FALSE ORDER BY created_at DESC LIMIT 1",
    )
    .bind(VerificationType::EmailConfirm)
    .bind(user_id)
    .fetch_optional(executor)
    .await
}

pub async fn increment_attempts<'e, E>(executor: E, id: Uuid) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE verifications SET attempts = attempts+1 WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await
        .map(|_| ())
}

pub async fn mark_as_used<'e, E>(executor: E, id: Uuid) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE verifications SET used = true WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await
        .map(|_| ())
}
