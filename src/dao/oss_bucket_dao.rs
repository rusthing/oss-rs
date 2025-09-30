use crate::id_worker::ID_WORKER;
use crate::model::oss_bucket::{Column, Entity, Model};
use crate::utils::time_utils::get_current_timestamp;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
};
use sea_orm::{ColumnTrait, QueryFilter};

/// 添加
pub async fn insert<C>(db: &C, mut model: Model) -> Result<Model, DbErr>
where
    C: ConnectionTrait,
{
    // 当id为默认值(0)时生成ID
    if model.id == 0 {
        model.id = ID_WORKER.get().unwrap().next_id() as i64;
    }
    // 当创建时间未设置时，设置创建时间和修改时间
    if model.create_timestamp == 0 {
        let now = get_current_timestamp() as i64;
        model.create_timestamp = now;
        model.update_timestamp = now;
    }
    let active_model = model.into_active_model();
    active_model.insert(db).await
}

/// 修改
pub async fn update<C>(db: &C, mut model: Model) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    // 当修改时间未设置时，设置修改时间
    if model.update_timestamp == 0 {
        let now = get_current_timestamp() as i64;
        model.update_timestamp = now;
    }
    let active_model = model.into_active_model();
    active_model.update(db).await?;
    Ok(())
}

/// 删除
pub async fn delete<C>(db: &C, model: Model) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    let active_model = model.into_active_model();
    active_model.delete(db).await?;
    Ok(())
}

/// 根据id查询
pub async fn get_by_id<C>(db: &C, id: i64) -> Result<Option<Model>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(id).one(db).await
}

/// 根据名称查询桶
pub async fn get_by_name<C>(db: &C, name: &str) -> Result<Option<Model>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find().filter(Column::Name.eq(name)).one(db).await
}
