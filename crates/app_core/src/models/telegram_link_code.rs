use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct TelegramLinkCode {
    pub id: Uuid,
    pub telegram_id: i64,
    pub telegram_username: Option<String>,
    pub telegram_first_name: Option<String>,
    pub telegram_photo_url: Option<String>,
    pub code: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub used_by_user_id: Option<Uuid>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
