use crate::dao::oss_obj_dao;
use crate::model::oss_obj::Model;
use crate::ro::ro::Ro;
use sea_orm::{DatabaseConnection, DbErr};

pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Ro<Model>, DbErr> {
    let result = oss_obj_dao::get_by_id(db, id).await?;
    Ok(Ro::success("查询成功".to_string()).extra(result))
}
