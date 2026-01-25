use crate::app::response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Database Error: {0}")]
    Db(sea_orm::DbErr),
}
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = axum::Json(ApiResponse::<()>::err(status, self.to_string()));
        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        ApiError::Db(err)
    }
}
