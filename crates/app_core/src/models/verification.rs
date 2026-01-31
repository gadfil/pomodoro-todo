use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::enums::VerificationType;

#[derive(sqlx::FromRow)]
pub struct Verification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: VerificationType,
    pub code: Option<String>,
    pub token: Option<String>,
    pub new_email: Option<String>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
