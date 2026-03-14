use crate::model::oss_bucket::ActiveModel;
use robotech_macros::crud_dto;

#[crud_dto]
pub struct OssBucketDto {
    /// 名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
}