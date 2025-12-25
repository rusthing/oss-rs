use crate::model::oss_bucket::Model;
use o2o::o2o;
use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none};
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, ToSchema, Debug, Serialize, Clone)]
#[from(Model)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OssBucketVo {
    /// ID
    #[from(~ as u64)]
    #[serde_as(as = "String")]
    pub id: u64,
    /// 名称
    #[from(~.to_string())]
    pub name: String,
    /// 创建者ID
    #[from(~ as u64)]
    #[serde_as(as = "String")]
    pub creator_id: u64,
    /// 创建时间
    #[from(~ as u64)]
    #[serde_as(as = "String")]
    pub create_timestamp: u64,
    /// 更新者ID
    #[from(~ as u64)]
    #[serde_as(as = "String")]
    pub updator_id: u64,
    /// 更新时间
    #[from(~ as u64)]
    #[serde_as(as = "String")]
    pub update_timestamp: u64,
}
