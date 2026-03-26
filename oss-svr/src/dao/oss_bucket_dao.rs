use robotech::macros::dao;

#[dao(
    unique_keys: [
        ("name", "桶名称"),
    ],
    like_columns: [
        Column::Name,
        Column::Remark
    ],
)]
pub struct OssBucketDao;
