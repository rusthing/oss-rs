use crate::model::oss_obj_ref::{ActiveModel, Column, Entity, Model};
use crate::model::{oss_bucket, oss_obj};
use robotech::macros::dao;
use sea_orm::{ColumnTrait, DeleteResult, QueryFilter};

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
