use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(ToSchema, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OssObjVo {
    /// ID
    pub id: String,
    /// 文件路径
    pub path: String,
    /// 文件Hash
    pub hash: String,
    /// 文件大小
    pub size: String,
    /// 访问URL地址
    pub url: String,
    /// 是否完成
    pub is_completed: bool,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间戳
    pub create_timestamp: String,
    /// 更新者ID
    pub updator_id: String,
    /// 更新时间戳
    pub update_timestamp: String,
}
