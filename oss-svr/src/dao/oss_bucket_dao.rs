use crate::model::oss_bucket::{ActiveModel, Column, Entity, Model};
use once_cell::sync::Lazy;
use robotech::dao::{push_unique_field, DaoError};
use robotech::macros::dao;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};
use std::collections::HashMap;

/// # 存储unique字段的HashMap
///
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
static UNIQUE_FIELDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut unique_fields = HashMap::new();
    push_unique_field(&mut unique_fields, "oss_bucket", "name", "桶名称");
    unique_fields
});

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
