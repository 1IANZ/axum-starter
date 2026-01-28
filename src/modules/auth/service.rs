// Service 层：封装认证相关业务逻辑
use crate::core::auth::{get_jwt, Principal};
use crate::core::error::{ApiError, ApiResult};
use crate::core::utils::verify_password;
use crate::modules::auth::dto::{LoginParams, LoginResult};
use crate::modules::auth::repo;
use sea_orm::DatabaseConnection;

pub async fn login(db: &DatabaseConnection, params: LoginParams) -> ApiResult<LoginResult> {
    // 从数据库加载用户
    let user = repo::find_user_by_account(db, &params.account)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;

    // 校验密码
    if !verify_password(&params.password, &user.password)? {
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }

    // 构建 JWT 并返回给客户端
    let principal = Principal {
        id: user.id,
        name: user.name,
    };
    let access_token = get_jwt().encode(principal)?;

    Ok(LoginResult { access_token })
}
