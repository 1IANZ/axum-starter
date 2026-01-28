mod dto;
mod handler;
mod repo;
mod service;

use crate::core::AppState;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    handler::create_router()
}
