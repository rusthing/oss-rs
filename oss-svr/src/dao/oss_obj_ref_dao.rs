use crate::model::oss_obj_ref::{ActiveModel, Column, Entity, Model};
use crate::model::{oss_bucket, oss_obj};
use once_cell::sync::Lazy;
use robotech::dao::{push_feign_key, push_unique_field, ForeignKey};
use robotech::macros::dao;
use robotech_macros::{define_foreign_keys, define_unique_fields};
use sea_orm::{ColumnTrait, DeleteResult, QueryFilter};
use std::collections::HashMap;

// 定义唯一字段列表
define_unique_fields! {
    "oss_obj_ref",
    ("url", "对象引用的URL"),
}

// 定义外键列表
define_foreign_keys! {
    "oss_obj_ref", "对象引用",
    ("bucket_id", "oss_bucket", "桶"),
    ("obj_id", "oss_obj", "对象"),
}

#[dao]
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
            .map_err(|e| DaoError::parse_db_err(e, &UNIQUE_FIELDS, &FOREIGN_KEYS))
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
    pub async fn get_by_id_also_related<C>(
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
            .map_err(|e| DaoError::parse_db_err(e, &UNIQUE_FIELDS, &FOREIGN_KEYS))
    }
}
