use dotenvy::dotenv;
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
}
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config load error {0}")]
    Load(#[from] config::ConfigError),
    #[error("Validation error {0:?}")]
    Validation(Vec<String>),
}
impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv().ok();

        let config: Config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        let mut errors = Vec::new();

        if self.database.url.is_empty() {
            errors.push("DATABASE__URL is required".to_string());
        }
        if self.database.max_connections == 0 {
            errors.push("DATABASE__MAX_CONNECTIONS must be > 0".to_string());
        }
        if self.database.min_connections > self.database.max_connections {
            errors.push("DATABASE__MIN_CONNECTIONS must be <= MAX_CONNECTIONS".to_string());
        }
        if self.server.port == 0 {
            errors.push("SERVER__PORT must be > 0".to_string());
        }
        if self.server.host.is_empty() {
            errors.push("SERVER__HOST is required".to_string());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Validation(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn valid_config() -> Config {
        Config {
            database: DatabaseConfig {
                url: "postgresql://localhost:5432/test".to_string(),
                max_connections: 10,
                min_connections: 2,
                acquire_timeout_secs: 5,
            },
            server: ServerConfig {
                port: 3000,
                host: "127.0.0.1".to_string(),
            },
        }
    }

    #[test]
    fn test_valid_config_passes_validation() {
        let config = valid_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_empty_database_url_fail() {
        let mut config = valid_config();
        config.database.url = "".to_string();
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_zero_max_connections_fails() {
        let mut config = valid_config();
        config.database.max_connections = 0;

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_min_greater_than_max_connections_fails() {
        let mut config = valid_config();
        config.database.min_connections = 20;
        config.database.max_connections = 10;

        let result = config.validate();
        assert!(result.is_err());

        if let Err(ConfigError::Validation(errors)) = result {
            assert!(errors.iter().any(|e| e.contains("MIN_CONNECTIONS")));
        }
    }

    #[test]
    fn test_zero_port_fails() {
        let mut config = valid_config();
        config.server.port = 0;

        let result = config.validate();
        assert!(result.is_err());
    }
}
