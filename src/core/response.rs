use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: u16, msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data,
        }
    }

    pub fn ok(msg: impl Into<String>, data: Option<T>) -> Self {
        Self::new(StatusCode::OK.as_u16(), msg, data)
    }

    pub fn err(code: StatusCode, msg: impl Into<String>) -> Self {
        Self::new(code.as_u16(), msg, None)
    }
}
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
