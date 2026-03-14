use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(ToSchema, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OssObjVo {
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
