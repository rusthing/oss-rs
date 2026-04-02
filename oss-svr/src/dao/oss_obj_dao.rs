use crate::model::oss_obj_ref::{Column as OssObjRefColumn, Entity as OssObjRefEntity};
use robotech::macros::dao;
use sea_orm::{ColumnTrait, QuerySelect, QueryTrait};

#[dao(
    unique_keys: [
        ("path", "对象路径"),
        ("size,hash", "对象大小与Hash"),
        ("url", "对象 URL"),
    ],
    like_columns: [
        Column::Path,
        Column::Hash,
    ],
)]
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
}
