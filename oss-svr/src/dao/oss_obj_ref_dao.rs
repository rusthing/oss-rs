use crate::mo::{oss_bucket, oss_obj};
use robotech::macros::dao;

/// 对象引用
///
/// ## 唯一约束
/// - `download_url`：同一份下载地址只留一条引用。约束名 `ak_download_url_oss_obj_ref`，
///   按框架的 `ak_{列名_and_连接}_{表名}` 生成，重复引用能回查到中文提示。
/// - `url`：旧约束名 `ak_url_oss_obj_ref` 的别名。这个 Key 的 code 原本是 `URL`
///   （约束其实建在 `download_url` 上），改名见
///   `migrations/pgsql/20261023193918_alter-table.sql`；还没跑到这条迁移的库，
///   PostgreSQL 报出来的仍是旧名，留住这条别名才不会退化成 500。
#[dao(
    unique_keys: [
        ("download_url", "对象引用的下载URL"),
        ("url", "对象引用的下载URL")
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
