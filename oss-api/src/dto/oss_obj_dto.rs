use robotech::macros::crud_dto;

#[crud_dto]
pub struct OssObjDto {
    /// 路径
    /// @len 1..=255
    pub path: String,
    /// 文件大小
    pub size: Option<u64>,
    /// Hash
    /// @len 0..=64
    pub hash: Option<String>,
    /// 是否完成
    #[db_default]
    pub completed: bool,
}