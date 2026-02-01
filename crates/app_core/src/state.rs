use crate::config::Config;
use crate::db::postgres::create_pool;
use crate::errors::StateError;
use crate::services::notification::CodeSender;
use sqlx::PgPool;
use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub code_sender: Arc<dyn CodeSender>,
}

impl AppState {
    pub async fn new(config: Config, code_sender: Arc<dyn CodeSender>) -> Result<Self, StateError> {
        let db = create_pool(&config.database).await?;
        Ok(Self {
            db,
            config: Arc::new(config),
            code_sender,
        })
    }
}
