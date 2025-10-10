use crate::id_worker::ID_WORKER;
use crate::model::oss_bucket::{ActiveModel, Column, Entity, Model};
use crate::utils::time_utils::get_current_timestamp;
use once_cell::sync::Lazy;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr, EntityTrait};
use sea_orm::{ColumnTrait, QueryFilter};
use std::collections::HashMap;

/// 存储unique字段的HashMap
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
pub static UNIQUE_FIELD_HASHMAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut hashmap = HashMap::new();
    hashmap.insert("name", "桶名称");
    hashmap
});

/// 添加
pub async fn insert<C>(mut active_model: ActiveModel, db: &C) -> Result<Model, DbErr>
where
    C: ConnectionTrait,
{
    // 当id为默认值(0)时生成ID
    if active_model.id == ActiveValue::NotSet {
        active_model.id = ActiveValue::set(ID_WORKER.get().unwrap().next_id() as i64);
    }
    // 当创建时间未设置时，设置创建时间和修改时间
    if active_model.create_timestamp == ActiveValue::NotSet {
        let now = ActiveValue::set(get_current_timestamp() as i64);
        active_model.create_timestamp = now.clone();
        active_model.update_timestamp = now;
    }
    // 添加时修改者就是创建者
    active_model.updator_id = active_model.creator_id.clone();
    // 执行数据库插入操作
    active_model.insert(db).await
}

/// 修改
pub async fn update<C>(mut active_model: ActiveModel, db: &C) -> Result<Model, DbErr>
where
    C: ConnectionTrait,
{
    // 当修改时间未设置时，设置修改时间
    if active_model.update_timestamp == ActiveValue::NotSet {
        let now = ActiveValue::set(get_current_timestamp() as i64);
        active_model.update_timestamp = now;
    }
    // 提取id以在更新后获取结果
    let id = active_model.id.clone().unwrap();
    // 执行数据库更新操作
    active_model.update(db).await?;
    Ok(get_by_id(id, db).await?.unwrap())
}

/// 删除
pub async fn delete<C>(active_model: ActiveModel, db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    active_model.delete(db).await?;
    Ok(())
}

/// 根据id查询
pub async fn get_by_id<C>(id: i64, db: &C) -> Result<Option<Model>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(id).one(db).await
}

/// 根据名称查询桶
pub async fn get_by_name<C>(name: &str, db: &C) -> Result<Option<Model>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find().filter(Column::Name.eq(name)).one(db).await
}
