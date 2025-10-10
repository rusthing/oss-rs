use crate::id_worker::ID_WORKER;
use crate::model::oss_obj_ref::{ActiveModel, Entity, Model};
use crate::model::{oss_bucket, oss_obj};
use crate::utils::time_utils::get_current_timestamp;
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    IntoActiveModel,
};
use sea_orm::{PaginatorTrait, QueryFilter};
use std::collections::HashMap;

/// 存储unique字段的HashMap
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
pub static UNIQUE_FIELD_HASHMAP: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| HashMap::new());

/// 添加
pub async fn insert<C>(mut model: ActiveModel, db: &C) -> Result<Model, DbErr>
where
    C: ConnectionTrait,
{
    // 当id为默认值(0)时生成ID
    if model.id == ActiveValue::NotSet {
        model.id = ActiveValue::set(ID_WORKER.get().unwrap().next_id() as i64);
    }
    // 当创建时间未设置时，设置创建时间和修改时间
    if model.create_timestamp == ActiveValue::NotSet {
        let now = ActiveValue::set(get_current_timestamp() as i64);
        model.create_timestamp = now.clone();
        model.update_timestamp = now;
    }
    let active_model = model.into_active_model();
    active_model.insert(db).await
}

/// 修改
pub async fn update<C>(
    mut model: ActiveModel,
    db: &C,
) -> Result<(Model, oss_bucket::Model, oss_obj::Model), DbErr>
where
    C: ConnectionTrait,
{
    // 当修改时间未设置时，设置修改时间
    if model.update_timestamp == ActiveValue::NotSet {
        let now = ActiveValue::set(get_current_timestamp() as i64);
        model.update_timestamp = now;
    }
    let id = model.id.clone().unwrap();
    let active_model = model.into_active_model();
    active_model.update(db).await?;
    Ok(get_by_id(id, db).await?.unwrap())
}

pub async fn delete<C>(model: ActiveModel, db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    let active_model = model.into_active_model();
    active_model.delete(db).await?;
    Ok(())
}

/// 根据id查询
pub async fn get_by_id<C>(
    id: i64,
    db: &C,
) -> Result<Option<(Model, oss_bucket::Model, oss_obj::Model)>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(id)
        .find_also_related(oss_bucket::Entity)
        .find_also_related(oss_obj::Entity)
        .one(db)
        .await
        .map(|model_option| {
            model_option.map(|(model, bucket_option, obj_option)| {
                (model, bucket_option.unwrap(), obj_option.unwrap())
            })
        })
}

/// 根据对象id查询数量
pub async fn count_by_obj_id<C>(obj_id: i64, db: &C) -> Result<u64, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find()
        .filter(<Entity as EntityTrait>::Column::ObjId.eq(obj_id))
        .count(db)
        .await
}
