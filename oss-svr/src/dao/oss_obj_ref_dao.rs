use crate::model::{oss_bucket, oss_obj};
use robotech::macros::dao;

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
