// Service 层：封装用户相关业务逻辑
use crate::core::common::{Page, PaginationParams};
use crate::core::error::{ApiError, ApiResult};
use crate::core::utils::encode_password;
use crate::modules::user::dto::UserParams;
use crate::modules::user::repo;
use crate::entity::sys_user;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};

pub async fn find_page(
    db: &DatabaseConnection,
    keyword: Option<String>,
    pagination: PaginationParams,
) -> ApiResult<Page<sys_user::Model>> {
    let (total, items) = repo::find_page(db, keyword.as_ref(), pagination.page, pagination.size).await?;
    Ok(Page::from_pagination(pagination, total, items))
}

pub async fn create(
    db: &DatabaseConnection,
    params: UserParams,
) -> ApiResult<sys_user::Model> {
    if params.password.is_empty() {
        return Err(ApiError::Biz(String::from("密码不能为空")));
    }
    let mut active_model = params.into_active_model();
    active_model.password =
        ActiveValue::Set(encode_password(&active_model.password.take().unwrap())?);
    repo::insert(db, active_model).await
}

pub async fn update(
    db: &DatabaseConnection,
    id: String,
    params: UserParams,
) -> ApiResult<sys_user::Model> {
    let existed_user = repo::find_by_id(db, &id)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待修改的用户不存在")))?;

    let old_password = existed_user.password.clone();
    let password = params.password.clone();
    let mut existed_active_model = existed_user.into_active_model();
    let mut active_model = params.into_active_model();
    existed_active_model.clone_from(&active_model);
    existed_active_model.id = ActiveValue::Unchanged(id);
    if password.is_empty() {
        existed_active_model.password = ActiveValue::Unchanged(old_password);
    } else {
        existed_active_model.password =
            ActiveValue::Set(encode_password(&active_model.password.take().unwrap())?);
    }

    repo::update(db, existed_active_model).await
}

pub async fn delete(db: &DatabaseConnection, id: String) -> ApiResult<()> {
    let existed_user = repo::find_by_id(db, &id)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待删除的用户不存在")))?;
    repo::delete(db, existed_user).await
}
