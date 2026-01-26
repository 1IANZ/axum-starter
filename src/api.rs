mod user;
use crate::app::AppState;
use crate::app::error::ApiError;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api", Router::new().nest("/user", user::create_router()))
        .fallback(|| async {
            tracing::warn!("Not Found");
            ApiError::NotFound
        })
        .method_not_allowed_fallback(|| async {
            tracing::warn!("Method Not Allowed");
            ApiError::MethodNotAllowed
        })
}
