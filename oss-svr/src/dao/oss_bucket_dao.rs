use crate::model::oss_bucket::{ActiveModel, Column, Entity, Model};
use once_cell::sync::Lazy;
use robotech::dao::{push_unique_field, ForeignKey};
use robotech::macros::dao;
use robotech_macros::{define_foreign_keys, define_unique_fields};
use sea_orm::{ColumnTrait, QueryFilter};
use std::collections::HashMap;

// 定义唯一字段列表
define_unique_fields! {
    "oss_bucket",
    ("name", "桶名称"),
}

// 定义外键列表
define_foreign_keys! {}

#[dao]
pub struct OssBucketDao;

impl OssBucketDao {
    /// # 根据名称查询相应记录
    ///
    /// 此函数负责根据提供的名称从数据库中查询对应的记录
    ///
    /// ## 参数
    /// * `name` - 要查询的记录的名称
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 查询成功，如果记录存在，返回查询到的完整 Model 实例，如果不存在返回None; 查询失败则返回相应的错误信息
    pub async fn get_by_name<C>(name: &str, db: &C) -> Result<Option<Model>, DaoError>
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Name.eq(name))
            .one(db)
            .await
            .map_err(DaoError::from)
    }
}
