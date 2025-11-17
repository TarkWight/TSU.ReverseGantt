use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        Ok(Config {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
                port: env::var("PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8080),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/reverse_gantt".into()),
            },
        })
    }
}