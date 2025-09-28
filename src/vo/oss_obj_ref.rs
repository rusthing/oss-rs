use crate::model::{oss_obj, oss_obj_ref};
use crate::vo::oss_obj::OssObjVo;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, Serialize)]
#[from((oss_obj_ref::Model, oss_obj::Model))]
#[serde(rename_all = "camelCase")]
pub struct OssObjRefVo {
    #[from(0,~.id.to_string())]
    pub id: String,
    #[from(0,~.obj_id.to_string())]
    pub obj_id: String,
    #[from(0,~.name.clone())]
    pub name: String,
    #[from(0,~.bucket.clone())]
    pub bucket: String,
    #[from(0,~.creator_id.clone().map(|v|v.to_string()))]
    pub creator_id: Option<String>,
    #[from(0,~.create_timestamp.clone().map(|v|v.to_string()))]
    pub create_timestamp: Option<String>,
    #[from(0,~.updator_id.clone().map(|v|v.to_string()))]
    pub updator_id: Option<String>,
    #[from(0,~.update_timestamp.clone().map(|v|v.to_string()))]
    pub update_timestamp: Option<String>,

    #[from(1,OssObjVo::from(~.clone()))]
    oss_obj: OssObjVo,
}
