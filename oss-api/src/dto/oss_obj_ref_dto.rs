use robotech::macros::crud_dto;

#[crud_dto]
pub struct OssObjRefDto {
    /// 对象ID
    pub obj_id: u64,
    /// 存储桶ID
    pub bucket_id: u64,
    /// 名称
    /// @len 1..=100
    pub name: String,
    /// 文件扩展名
    /// @len 0..=10
    pub ext: Option<String>,
    /// 下载URL
    /// @len 1..=200
    pub download_url: String,
    /// 预览URL
    /// @len 0..=200
    pub preview_url: Option<String>,
    /// 启用
    #[db_default]
    pub enabled: bool,
}