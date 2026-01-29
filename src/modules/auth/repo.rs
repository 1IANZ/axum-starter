use crate::core::error::ApiResult;
use crate::entity::prelude::*;
use crate::entity::sys_user;
use sea_orm::prelude::*;

pub async fn find_user_by_account(
    db: &DatabaseConnection,
    account: &str,
) -> ApiResult<Option<sys_user::Model>> {
    Ok(SysUser::find()
        .filter(sys_user::Column::Account.eq(account))
        .one(db)
        .await?)
}
