use crate::config::jwt::JwtConfig;
use anyhow::Context;
use config::Config;
pub use database::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::LazyLock;

mod database;
mod jwt;
mod server;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    jwt: JwtConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("Application")
                    .format(config::FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config"))
    }
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
    pub fn jwt(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
