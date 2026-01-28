pub mod auth;
pub mod user;
use crate::core::AppState;
use crate::core::error::ApiError;
use crate::core::middleware::get_auth_layer;
use axum::Router;
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/users", user::create_router())
                .route_layer(get_auth_layer())
                .nest("/auth", auth::create_router())
                .fallback(|| async {
                    tracing::warn!("Not Found");
                    ApiError::NotFound
                }),
        )
        .method_not_allowed_fallback(|| async {
            tracing::warn!("Method Not Allowed");
            ApiError::MethodNotAllowed
        })
}
