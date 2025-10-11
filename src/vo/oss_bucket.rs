use crate::model::oss_bucket::Model;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, ToSchema, Debug, Serialize)]
#[from(Model)]
#[serde(rename_all = "camelCase")]
pub struct OssBucketVo {
    /// ID
    #[from(~.to_string())]
    pub id: String,
    /// 名称
    #[from(~.to_string())]
    pub name: String,
    /// 创建者ID
    #[from(~.to_string())]
    pub creator_id: String,
    /// 创建时间
    #[from(~.to_string())]
    pub create_timestamp: String,
    /// 更新者ID
    #[from(~.to_string())]
    pub updator_id: String,
    /// 更新时间
    #[from(~.to_string())]
    pub update_timestamp: String,
}
