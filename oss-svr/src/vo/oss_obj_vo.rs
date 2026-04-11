use crate::model::oss_obj::Model;
use robotech::macros::vo;

#[vo]
pub struct OssObjVo {
    /// ID
    pub id: u64,
    /// 文件路径
    pub path: String,
    /// 文件大小
    pub size: Option<u64>,
    /// 文件Hash
    pub hash: Option<String>,
    /// 是否完成
    pub is_completed: bool,
    /// 创建者ID
    pub creator_id: u64,
    /// 创建时间戳
    pub create_timestamp: u64,
    /// 更新者ID
    pub updator_id: u64,
    /// 更新时间戳
    pub update_timestamp: u64,
}
