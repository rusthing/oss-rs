use crate::dao::oss_obj_dao;
use crate::model::oss_obj::Model;
use crate::ro::ro::Ro;
use sea_orm::DatabaseConnection;

pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Ro<Option<Model>> {
    let model = oss_obj_dao::get_by_id(db, id).await.unwrap();
    Ro::success("查询成功".to_string()).extra(model)
}
