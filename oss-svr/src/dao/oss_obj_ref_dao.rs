use crate::model::oss_obj_ref::{ActiveModel, Column, Entity, Model};
use crate::model::{oss_bucket, oss_obj};
use once_cell::sync::Lazy;
use robotech::dao::DaoError;
use robotech::macros::dao;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait,
    QueryFilter,
};
use std::collections::HashMap;

/// 存储unique字段的HashMap
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
pub static UNIQUE_FIELDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());

#[dao(exclude: get_by_id)]
pub struct OssObjRefDao;

impl OssObjRefDao {
    pub async fn delete_by_bucket_id<C>(bucket_id: u64, db: &C) -> Result<DeleteResult, DaoError>
    where
        C: ConnectionTrait,
    {
        Entity::delete_many()
            .filter(Column::BucketId.eq(bucket_id))
            .exec(db)
            .await
            .map_err(|e| DaoError::parse_db_err(e, &UNIQUE_FIELDS))
    }

    /// # 根据ID查询记录
    ///
    /// 此函数通过给定的ID查询单条记录，并同时获取关联的存储桶和对象信息
    ///
    /// ## 参数
    /// * `id` - 要查询的记录的唯一标识符
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回一个包含主记录、关联存储桶和关联对象的元组的Option，如果查询失败则返回相应的错误信息
    /// 如果未找到匹配记录，则返回None
    pub async fn get_by_id<C>(
        id: u64,
        db: &C,
    ) -> Result<Option<(Model, oss_bucket::Model, oss_obj::Model)>, DaoError>
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(id as i64)
            .find_also_related(oss_bucket::Entity)
            .find_also_related(oss_obj::Entity)
            .one(db)
            .await
            .map(|model_option| {
                model_option.map(|(model, bucket_option, obj_option)| {
                    (model, bucket_option.unwrap(), obj_option.unwrap())
                })
            })
            .map_err(|e| DaoError::parse_db_err(e, &UNIQUE_FIELDS))
    }
}
