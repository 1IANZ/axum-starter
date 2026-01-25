use anyhow::Context;
use config::Config;
use serde::Deserialize;
use std::sync::LazyLock;
pub mod database;
pub mod server;
pub use database::DatabaseConfig;
pub use server::ServerConfig;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
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
    pub fn server(&self) -> &server::ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &database::DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
