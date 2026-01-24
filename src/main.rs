mod config;
mod database;
mod entity;
mod logger;

use axum::{Router, debug_handler, extract::State, routing::get};
use entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;
    let router = Router::new()
        .route("/", get(index))
        .route("/users", get(query_users))
        .with_state(db);
    let port = config::get().server().port();
    let listener = TcpListener::bind(format!("127.0.0.1:{port}",)).await?;
    tracing::info!("listening on http://127.0.0.1:{port}");
    axum::serve(listener, router).await?;
    Ok(())
}
#[debug_handler]
async fn index() -> &'static str {
    "Hello, World!"
}
#[debug_handler]
async fn query_users(
    State(db): State<DatabaseConnection>,
) -> axum::Json<Vec<entity::sys_user::Model>> {
    let users = SysUser::find().all(&db).await.unwrap();
    axum::Json(users)
}
