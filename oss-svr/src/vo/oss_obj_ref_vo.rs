use crate::model::oss_obj_ref::Model;
use robotech_macros::vo;

#[vo]
pub struct OssObjRefVo {
    /// ID
    pub id: u64,
    /// 对象ID
    pub obj_id: u64,
    /// 存储桶ID
    pub bucket_id: u64,
    /// 名称
    pub name: String,
    /// 扩展名
    pub ext: String,
    /// 访问URL地址
    pub url: String,
    /// 创建者ID
    pub creator_id: u64,
    /// 创建时间戳
    pub create_timestamp: u64,
    /// 更新者ID
    pub updator_id: u64,
    /// 更新时间戳
    pub update_timestamp: u64,
}
