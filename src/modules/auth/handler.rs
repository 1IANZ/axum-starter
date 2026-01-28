// Handler 层：负责参数解析与响应包装
use crate::core::auth::Principal;
use crate::core::error::ApiResult;
use crate::core::middleware::get_auth_layer;
use crate::core::response::ApiResponse;
use crate::core::extract::valid::ValidJson;
use crate::core::AppState;
use crate::modules::auth::dto::{LoginParams, LoginResult};
use crate::modules::auth::service;
use axum::extract::{ConnectInfo, State};
use axum::{debug_handler, routing, Extension, Router};
use std::net::SocketAddr;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.account, ip = %addr.ip()))]
async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<ApiResponse<LoginResult>> {
    tracing::info!("开始处理登录逻辑...");
    let result = service::login(&db, params).await?;
    tracing::info!("登录成功, JWT Token: {}", result.access_token);

    Ok(ApiResponse::ok("登录成功", Some(result)))
}

#[debug_handler]
async fn get_user_info(
    Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>> {
    // 已由 JWT 中间件完成身份注入
    Ok(ApiResponse::ok("ok", Some(principal)))
}
