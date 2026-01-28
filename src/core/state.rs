use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    // 全局共享状态（数据库连接池）
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
