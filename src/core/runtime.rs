use crate::config;
use crate::core::AppState;
use crate::core::logger;
use crate::infra::database;
use crate::infra::server;
use axum::Router;

// 应用启动入口
pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    // 初始化日志与 ID
    logger::init();
    crate::core::id::init()?;
    tracing::info!("Starting app server...");
    // 初始化数据库连接池
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());
    server.start(state, router).await
}
