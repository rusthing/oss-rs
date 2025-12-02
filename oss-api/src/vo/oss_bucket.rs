use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(Serialize)]的上方才能起效)
#[derive(ToSchema, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OssBucketVo {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub create_timestamp: String,
    /// 更新者ID
    pub updator_id: String,
    /// 更新时间
    pub update_timestamp: String,
}
