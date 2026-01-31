use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use uuid::Uuid;
use crate::enums::{AuthEventType, OAuthProvider};

#[derive(sqlx::FromRow)]
pub struct AuthEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: AuthEventType,
    pub provider: Option<OAuthProvider>,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
