use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::enums::UserStatus;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub email_verified: bool,
    pub password_hash: Option<String>,
    pub username: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub is_anonymous: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
