use crate::model::oss_obj::{Entity, Model};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id as i64).one(db).await
}
