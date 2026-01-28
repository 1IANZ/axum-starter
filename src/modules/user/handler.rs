// Handler 层：参数解析与响应包装
use crate::core::common::Page;
use crate::core::error::ApiResult;
use crate::core::extract::path::Path;
use crate::core::response::ApiResponse;
use crate::core::extract::valid::{ValidJson, ValidQuery};
use crate::core::AppState;
use crate::entity::sys_user;
use crate::modules::user::dto::{UserParams, UserQueryParams};
use crate::modules::user::service;
use axum::{debug_handler, extract::State, routing, Router};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::delete(delete))
}

#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams { keyword, pagination }): ValidQuery<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<sys_user::Model>>> {
    let page = service::find_page(&db, keyword, pagination).await?;
    Ok(ApiResponse::ok("ok", Some(page)))
}

#[debug_handler]
async fn create(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<sys_user::Model>> {
    let result = service::create(&db, params).await?;
    Ok(ApiResponse::ok("ok", Some(result)))
}

#[debug_handler]
async fn update(
    State(AppState { db }): State<AppState>,
    Path(id): Path<String>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<sys_user::Model>> {
    let result = service::update(&db, id, params).await?;
    Ok(ApiResponse::ok("ok", Some(result)))
}

#[debug_handler]
async fn delete(
    State(AppState { db }): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<ApiResponse<()>> {
    service::delete(&db, id).await?;
    Ok(ApiResponse::ok("ok", None))
}
