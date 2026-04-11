use robotech::macros::crud_dto;

#[crud_dto]
pub struct OssObjDto {
    /// 路径
    pub path: String,
    /// 文件大小
    pub size: Option<u64>,
    /// Hash
    pub hash: Option<String>,
    /// 是否完成
    pub is_completed: bool,
}
