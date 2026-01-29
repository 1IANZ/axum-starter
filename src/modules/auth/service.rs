use crate::core::auth::{Principal, get_jwt};
use crate::core::error::{ApiError, ApiResult};
use crate::core::utils::verify_password;
use crate::modules::auth::dto::{LoginParams, LoginResult};
use crate::modules::auth::repo;
use sea_orm::DatabaseConnection;

pub async fn login(db: &DatabaseConnection, params: LoginParams) -> ApiResult<LoginResult> {
    let user = repo::find_user_by_account(db, &params.account)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;

    if !verify_password(&params.password, &user.password)? {
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }
    let principal = Principal {
        id: user.id,
        name: user.name,
    };
    let access_token = get_jwt().encode(principal)?;

    Ok(LoginResult { access_token })
}
