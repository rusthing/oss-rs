use crate::dao::oss_bucket_dao;
use crate::dao::oss_bucket_dao::UNIQUE_FIELD_HASHMAP;
use crate::db::DB_CONN;
use crate::model::oss_bucket::ActiveModel;
use crate::ro::ro::Ro;
use crate::svc::svc_utils::{handle_db_err_to_svc_error, SvcError};
use crate::to::oss_bucket::{OssBucketAddTo, OssBucketModifyTo, OssBucketSaveTo};
use crate::vo::oss_bucket::OssBucketVo;
use sea_orm::DatabaseConnection;

/// # 根据id获取桶信息
///
/// 通过给定的ID从数据库中查询对应的桶记录，如果找到则返回封装在Ro中的Vo对象，否则返回NotFound错误。
///
/// ## 参数
/// * `id` - 要查询的桶的ID
/// * `db` - 数据库连接，如果未提供则使用全局数据库连接
///
/// ## 返回值
/// * `Ok(Ro<OssBucketVo>)` - 查询成功，返回封装在Ro中的OssBucketVo对象
/// * `Err(SvcError)` - 查询失败，可能是因为记录不存在或其他数据库错误
pub async fn get_by_id(
    id: u64,
    db: Option<&DatabaseConnection>,
) -> Result<Ro<OssBucketVo>, SvcError> {
    let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
    let one = oss_bucket_dao::get_by_id(id as i64, db).await?;
    Ok(Ro::success("查询成功".to_string()).extra(match one {
        Some(one) => Some(OssBucketVo::from(one)),
        _ => return Err(SvcError::NotFound(format!("id: {}", id))),
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

/// 保存
/// 如果id存在则修改，否则添加
pub async fn save(
    save_to: OssBucketSaveTo,
    db: Option<&DatabaseConnection>,
) -> Result<Ro<OssBucketVo>, SvcError> {
    if save_to.id.clone().is_some() {
        modify(save_to.into(), db).await
    } else {
        add(save_to.into(), db).await
    }
}

// 删除
// pub async fn del(obj_ref_id: u64) -> Result<Ro<()>, SvcError> {
// }
