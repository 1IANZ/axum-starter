// Repo 层：集中处理数据库读写
use crate::core::error::ApiResult;
use crate::entity::prelude::*;
use crate::entity::sys_user;
use sea_orm::prelude::*;
use sea_orm::{Condition, QueryOrder, QueryTrait};

pub async fn find_page(
    db: &DatabaseConnection,
    keyword: Option<&String>,
    page: u64,
    size: u64,
) -> ApiResult<(u64, Vec<sys_user::Model>)> {
    let paginator = SysUser::find()
        .apply_if(keyword, |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(db, size);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page - 1).await?;
    Ok((total, items))
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: &str,
) -> ApiResult<Option<sys_user::Model>> {
    Ok(SysUser::find_by_id(id).one(db).await?)
}

pub async fn insert(
    db: &DatabaseConnection,
    active_model: sys_user::ActiveModel,
) -> ApiResult<sys_user::Model> {
    Ok(active_model.insert(db).await?)
}

pub async fn update(
    db: &DatabaseConnection,
    active_model: sys_user::ActiveModel,
) -> ApiResult<sys_user::Model> {
    Ok(active_model.update(db).await?)
}

pub async fn delete(db: &DatabaseConnection, model: sys_user::Model) -> ApiResult<()> {
    model.delete(db).await?;
    Ok(())
}
