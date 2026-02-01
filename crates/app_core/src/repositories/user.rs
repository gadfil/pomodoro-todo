use crate::models::user::User;
use sqlx::{Executor, Postgres};
use uuid::Uuid;

pub async fn find_by_email<'e, E>(executor: E, email: &str) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(executor)
        .await
}

pub async fn set_email_verified<'e, E>(executor: E, id: Uuid) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE users SET email_verified=true WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await
        .map(|_| ())
}
pub async fn create<'e, E>(
    executor: E,
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<User, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, display_name) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .fetch_one(executor)
    .await
}
