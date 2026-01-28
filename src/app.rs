pub mod common;
mod database;
pub mod error;
pub mod json;
mod latency;
mod logger;
pub mod path;
pub mod query;
pub mod response;
mod serde;
mod server;
pub mod valid;
pub mod validation;
pub mod id;

use crate::config;
use axum::Router;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    id::init()?;
    tracing::info!("Starting app server...");
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());
    server.start(state, router).await
}
