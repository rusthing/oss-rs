use crate::model::{oss_bucket, oss_obj, oss_obj_ref};
use crate::vo::oss_bucket::OssBucketVo;
use crate::vo::oss_obj::OssObjVo;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, Debug, Serialize)]
#[from((oss_obj_ref::Model, oss_bucket::Model, oss_obj::Model))]
#[serde(rename_all = "camelCase")]
pub struct OssObjRefVo {
    #[from(0,~.id.to_string())]
    pub id: String,
    #[from(0,~.name.to_string())]
    pub name: String,
    #[from(0,~.ext.to_string())]
    pub ext: String,
    #[from(0,~.creator_id.to_string())]
    pub creator_id: String,
    #[from(0,~.create_timestamp.to_string())]
    pub create_timestamp: String,
    #[from(0,~.updator_id.to_string())]
    pub updator_id: String,
    #[from(0,~.update_timestamp.to_string())]
    pub update_timestamp: String,

    #[from(1,OssBucketVo::from(~.clone()))]
    oss_bucket: OssBucketVo,
    #[from(2,OssObjVo::from(~.clone()))]
    oss_obj: OssObjVo,
}
