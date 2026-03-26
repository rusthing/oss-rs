use robotech_macros::crud_dto;

#[crud_dto]
pub struct OssObjRefDto {
    /// 对象ID
    pub obj_id: u64,
    /// 存储桶ID
    pub bucket_id: u64,
    /// 名称
    pub name: String,
    /// 文件扩展名
    pub ext: Option<String>,
    /// 下载URL
    pub download_url: String,
    /// 预览URL
    pub preview_url: Option<String>,
}
