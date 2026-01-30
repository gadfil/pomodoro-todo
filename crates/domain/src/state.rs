use sqlx::PgPool;
use std::sync::Arc;
use shared::config::Config;
use infrastructure::db::postgres::create_pool;
use crate::errors::StateError;
#[derive(Clone)]
pub struct  AppState{
    pub db: PgPool,
    pub config: Arc<Config>,
}

impl AppState {
   pub async fn new( config:Config) -> Result<Self, StateError> {
       let db= create_pool(&config.database).await?;
       Ok(Self {
           db,
           config:Arc::new(config)
       })
   }
}