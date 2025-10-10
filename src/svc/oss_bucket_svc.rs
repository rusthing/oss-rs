use crate::dao::oss_bucket_dao;
use crate::dao::oss_bucket_dao::UNIQUE_FIELD_HASHMAP;
use crate::db::DB_CONN;
use crate::model::oss_bucket::ActiveModel;
use crate::ro::ro::Ro;
use crate::svc::svc_utils::{handle_db_err_to_svc_error, SvcError};
use crate::to::oss_bucket::{OssBucketAddTo, OssBucketModifyTo};
use crate::vo::oss_bucket::OssBucketVo;
use sea_orm::DatabaseConnection;

/// 根据id获取对象信息
pub async fn get_by_id(
    obj_ref_id: u64,
    db: Option<&DatabaseConnection>,
) -> Result<Ro<OssBucketVo>, SvcError> {
    let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
    let one = oss_bucket_dao::get_by_id(obj_ref_id as i64, db).await?;
    Ok(Ro::success("查询成功".to_string()).extra(match one {
        Some(one) => Some(OssBucketVo::from(one)),
        _ => return Err(SvcError::NotFound(format!("id: {}", obj_ref_id))),
    }))
}

/// 添加
pub async fn add(
    add_to: OssBucketAddTo,
    db: Option<&DatabaseConnection>,
) -> Result<Ro<OssBucketVo>, SvcError> {
    let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
    let active_model: ActiveModel = add_to.into();
    let one = oss_bucket_dao::insert(active_model, db)
        .await
        .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELD_HASHMAP))?;
    Ok(Ro::success("添加成功".to_string()).extra(Some(OssBucketVo::from(one))))
}

/// 修改
pub async fn modify(
    modify_to: OssBucketModifyTo,
    db: Option<&DatabaseConnection>,
) -> Result<Ro<OssBucketVo>, SvcError> {
    let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
    let id = modify_to.id.clone().unwrap().parse::<u64>().unwrap();
    let active_model: ActiveModel = modify_to.into();
    oss_bucket_dao::update(active_model, db)
        .await
        .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELD_HASHMAP))?;
    Ok(get_by_id(id, Some(db)).await?.msg("修改成功".to_string()))
}

// 删除
// pub async fn del(obj_ref_id: u64) -> Result<Ro<()>, SvcError> {
// }
