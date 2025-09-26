use crate::id_worker::ID_WORKER;
use crate::model::oss_obj;
use crate::model::oss_obj_ref::{Entity, Model};
use crate::utils::time_utils::get_current_timestamp;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
};
use sea_orm::{PaginatorTrait, QueryFilter};

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
    if model.create_timestamp.is_none() {
        let now = Some(get_current_timestamp() as i64);
        model.create_timestamp = now;
        model.update_timestamp = now;
    }
    let active_model = model.into_active_model();
    active_model.insert(db).await
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
pub async fn get_by_id<C>(db: &C, id: i64) -> Result<Option<(Model, Option<oss_obj::Model>)>, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(id)
        .find_also_related(oss_obj::Entity)
        .one(db)
        .await
}

/// 根据对象id查询数量
pub async fn count_by_obj_id<C>(db: &C, obj_id: i64) -> Result<u64, DbErr>
where
    C: ConnectionTrait,
{
    Entity::find()
        .filter(<Entity as EntityTrait>::Column::ObjId.eq(obj_id))
        .count(db)
        .await
}
