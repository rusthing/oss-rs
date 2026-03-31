use crate::model::{oss_bucket, oss_obj};
use robotech::macros::dao;
use sea_orm::ColumnTrait;

/// 对象引用
#[dao(
    unique_keys: [
        ("url", "对象引用的URL")
    ],
    foreign_keys: [
        ("bucket_id", "oss_bucket", "桶"),
        ("obj_id", "oss_obj", "对象")
    ],
    like_columns: [
        Column::Name,
        Column::DownloadUrl,
        Column::PreviewUrl
    ],
    related_table: [
        "oss_bucket",
        "oss_obj"
    ]
)]
pub struct OssObjRefDao;

impl OssObjRefDao {
    // /// # 删除记录
    // ///
    // /// 根据提供的查询参数删除数据库中的记录
    // ///
    // /// ## 参数
    // /// - `condition`: 查询条件
    // /// - `db`: 数据库连接，如果未提供则使用全局数据库连接
    // ///
    // /// ## 返回值
    // /// - `Result<DeleteResult, DaoError>` - 删除结果
    // pub async fn delete_by_condition<C>(
    //     condition: Condition,
    //     db: &C,
    // ) -> Result<DeleteResult, DaoError>
    // where
    //     C: ConnectionTrait,
    // {
    //     Entity::delete_many()
    //         .filter(condition)
    //         .exec(db)
    //         .await
    //         .map_err(|e| DaoError::parse_db_err(e))
    // }

    pub async fn delete_by_bucket_id<C>(bucket_id: u64, db: &C) -> Result<DeleteResult, DaoError>
    where
        C: ConnectionTrait,
    {
        Entity::delete_many()
            .filter(Column::BucketId.eq(bucket_id))
            .exec(db)
            .await
            .map_err(|e| DaoError::parse_db_err(e))
    }
}
