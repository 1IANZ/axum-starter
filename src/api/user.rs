use crate::app::response::ApiResponse;
use crate::app::{AppState, error::ApiResult};

use crate::entity::{prelude::*, sys_user};
use axum::{Router, debug_handler, extract::State, routing};
use sea_orm::prelude::*;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(query_users))
}

#[debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find().all(&db).await?;

    Ok(ApiResponse::ok("ok", Some(users)))
}
