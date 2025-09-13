use crate::dao::oss_obj_dao;
use crate::ro::ro::Ro;
use sea_orm::DatabaseConnection;

pub async fn get_user_by_id(db: &DatabaseConnection, id: u64) -> Ro {
    let model = oss_obj_dao::get_by_id(db, id).await.unwrap();
    Ro::new()
}
