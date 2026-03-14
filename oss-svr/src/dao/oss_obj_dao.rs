use crate::model::oss_obj::{ActiveModel, Column, Entity, Model};
use crate::model::oss_obj_ref::{Column as OssObjRefColumn, Entity as OssObjRefEntity};
use linkme::distributed_slice;
use robotech::macros::dao;
use robotech_macros::define_unique_fields;
use sea_orm::{ColumnTrait, QueryFilter, QuerySelect, QueryTrait};

// 定义唯一键字段列表
define_unique_fields! {
    "oss_obj",
    ("path", "对象路径"),
    ("size,hash", "对象大小与Hash"),
    ("url", "对象 URL"),
}

#[dao]
pub struct OssObjDao;

impl OssObjDao {
    /// # 获取孤立没有关联对象引用的记录
    ///
    /// 此函数负责获取那些在 `oss_obj_ref` 表中没有关联记录的 `oss_obj` 记录。
    /// 这有助于清理孤立的数据，释放存储空间。
    ///
    /// ## 参数
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回查询到的记录列表
    pub async fn find_orphaned<C>(db: &C) -> Result<Vec<Model>, DaoError>
    where
        C: ConnectionTrait,
    {
        // 使用子查询删除没有关联记录的oss_obj记录
        Entity::find()
            .filter(
                Column::Id.not_in_subquery(
                    OssObjRefEntity::find()
                        .select_only()
                        .column(OssObjRefColumn::ObjId)
                        .into_query(),
                ),
            )
            .all(db)
            .await
            .map_err(|e| DaoError::parse_db_err(e))
    }

    /// # 根据哈希值和大小查询记录
    ///
    /// 此函数负责根据提供的哈希值和文件大小从数据库中查询对应的记录。
    /// 这通常用于检测是否已存在具有相同内容的文件，以避免重复存储。
    ///
    /// ## 参数
    /// * `hash` - 文件的哈希值（通常是MD5或SHA256等摘要算法的结果）
    /// * `size` - 文件的大小（以字节为单位）
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回查询到的完整 Model 实例（如果存在），如果查询失败则返回相应的错误信息
    pub async fn get_by_hash_and_size<C>(
        hash: &str,
        size: u64,
        db: &C,
    ) -> Result<Option<Model>, DaoError>
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Hash.eq(hash))
            .filter(Column::Size.eq(size))
            .one(db)
            .await
            .map_err(|e| DaoError::parse_db_err(e))
    }
}
