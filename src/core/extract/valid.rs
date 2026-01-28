use crate::core::error::ApiError;
use crate::core::extract::json::Json;
// use crate::core::extract::path::Path;
use crate::core::extract::query::Query;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;

#[derive(Debug, Clone, Copy, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);

#[derive(Debug, Clone, Default)]
pub struct ValidQuery<T>(pub T);
// #[derive(Debug, Clone, Default)]
// pub struct ValidPath<T>(pub T);
#[derive(Debug, Clone, Default)]
pub struct ValidJson<T>(pub T);

macro_rules! impl_from_request {
    ($name:ident, $wrapper:ident, FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequestParts<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                // 统一将校验过的值转换为业务类型
                Ok($name(Valid::from_request_parts(parts, state).await?.0.0))
            }
        }
    };
    ($name:ident, $wrapper:ident, FromRequest) => {
        impl<S, T> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequest<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
                // Body 类型走 FromRequest
                Ok($name(Valid::from_request(request, state).await?.0.0))
            }
        }
    };
}

impl_from_request!(ValidQuery, Query, FromRequestParts);
// impl_from_request!(ValidPath, Path, FromRequestParts);
impl_from_request!(ValidJson, Json, FromRequest);
