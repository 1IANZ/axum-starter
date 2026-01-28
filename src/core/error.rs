use crate::core::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_valid::ValidRejection;
use bcrypt::BcryptError;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Biz(String),
    #[error("服务器迷路了")]
    NotFound,
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("请求方法不支持")]
    MethodNotAllowed,
    #[error("数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("查询参数错误: {0}")]
    Query(#[from] QueryRejection),
    #[error("查询路径错误: {0}")]
    Path(#[from] PathRejection),
    #[error("Body参数错误: {0}")]
    Json(#[from] JsonRejection),
    #[error("参数校验错误: {0}")]
    Validation(String),
    #[error("密码Hash错误:{0}")]
    Bcrypt(#[from] BcryptError),
    #[error("JWT错误: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("未授权: {0}")]
    Unauthenticated(String),
}
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Internal(_) | ApiError::Db(_) | ApiError::Bcrypt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::JWT(_) | ApiError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            ApiError::Biz(_) => StatusCode::OK,
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

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            // 参数校验错误在此统一转换
            ValidRejection::Valid(errors) => ApiError::Validation(errors.to_string()),
            ValidRejection::Inner(errors) => errors,
        }
    }
}
impl From<ApiError> for Response {
    fn from(value: ApiError) -> Self {
        value.into_response()
    }
}
