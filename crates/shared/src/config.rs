use serde::Deserialize;
use config::ConfigError;
use dotenvy::dotenv;
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig

}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}


#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenv().ok();
        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true)
            ).build()?;
        config.try_deserialize()
    }
}