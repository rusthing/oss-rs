use crate::model::oss_obj::Model;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::string::ToString;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, Serialize)]
#[from(Model)]
#[serde(rename_all = "camelCase")]
pub struct OssObjVo {
    #[from(~.to_string())]
    pub id: String,
    #[from(~.to_string())]
    pub hash: String,
    #[from(~.to_string())]
    pub size: String,
    #[from(~.to_string())]
    pub path: String,
    #[from(~.to_string())]
    pub url: String,
    pub is_completed: bool,
    #[from(~.to_string())]
    pub creator_id: String,
    #[from(~.to_string())]
    pub create_timestamp: String,
    #[from(~.to_string())]
    pub updator_id: String,
    #[from(~.to_string())]
    pub update_timestamp: String,
}
