use crate::model::oss_bucket::Model;
use robotech::macros::vo;

#[vo]
pub struct OssBucketVo {
    /// ID
    pub id: u64,
    /// 名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 创建者ID
    pub creator_id: u64,
    /// 创建时间
    pub create_timestamp: u64,
    /// 更新者ID
    pub updator_id: u64,
    /// 更新时间
    pub update_timestamp: u64,
}
