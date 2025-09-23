use crate::model::oss_obj::Model;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::string::ToString;

#[skip_serializing_none]    // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, Serialize)]
#[from(Model)]
#[serde(rename_all = "camelCase")]
pub struct OssObjVo {
    #[from(~.to_string())]
    pub id: String,
    #[from(~.to_string())]
    pub name: String,
    #[from(~.clone().map(|v|v.to_string()))]
    pub ext: Option<String>,
    #[from(~.to_string())]
    pub bucket: String,
    #[from(~.clone().map(|v|v.to_string()))]
    pub hash: Option<String>,
    #[from(~.to_string())]
    pub ref_count: String,
    #[from(~.clone().map(|v|v.to_string()))]
    pub size: Option<String>,
    #[from(~.clone().map(|v|v.to_string()))]
    pub path: Option<String>,
    #[from(~.clone().map(|v|v.to_string()))]
    pub url: Option<String>,
    pub is_completed: bool,
    #[from(~.map(|v|v.to_string()))]
    pub creator_id: Option<String>,
    #[from(~.map(|v|v.to_string()))]
    pub create_timestamp: Option<String>,
    #[from(~.map(|v|v.to_string()))]
    pub updator_id: Option<String>,
    #[from(~.map(|v|v.to_string()))]
    pub update_timestamp: Option<String>,
}
