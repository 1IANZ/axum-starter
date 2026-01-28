use crate::core::common::PaginationParams;
use crate::core::enumeration::Gender;
use crate::entity::sys_user::ActiveModel;
use sea_orm::prelude::Date;
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    pub keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    #[validate(length(min = 1, max = 16, message = "姓名长度为1-16"))]
    pub name: String,
    pub gender: Gender,
    #[validate(length(min = 1, max = 16, message = "账号长度为1-16"))]
    pub account: String,
    #[validate(length(max = 16, message = "密码长度为6-16"))]
    pub password: String,
    #[validate(custom(function = "crate::core::validation::is_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}
