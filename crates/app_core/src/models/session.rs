use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_name: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
}
