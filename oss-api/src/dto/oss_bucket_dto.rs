use robotech::macros::crud_dto;

#[crud_dto]
pub struct OssBucketDto {
    /// 名称
    /// @len 1..=50
    pub name: String,
    /// 备注
    /// @len 0..=50
    pub remark: Option<String>,
    /// 启用
    #[db_default]
    pub enabled: bool,
}