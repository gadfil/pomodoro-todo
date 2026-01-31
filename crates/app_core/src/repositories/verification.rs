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
