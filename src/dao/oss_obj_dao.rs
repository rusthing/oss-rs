use crate::model::oss_obj::{Entity, Model};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel,
};

/// 添加
pub async fn insert(db: &DatabaseTransaction, model: Model) -> Result<Model, DbErr> {
    let active_model = model.into_active_model();
    active_model.insert(db).await
}

/// 根据id查询
pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id as i64).one(db).await
}
