use crate::model::{oss_bucket, oss_obj, oss_obj_ref};
use crate::vo::oss_bucket::OssBucketVo;
use crate::vo::oss_obj::OssObjVo;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, ToSchema, Debug, Serialize, Clone)]
#[from((oss_obj_ref::Model, oss_bucket::Model, oss_obj::Model))]
#[serde(rename_all = "camelCase")]
pub struct OssObjRefVo {
    /// ID
    #[from(0,~.id.to_string())]
    pub id: String,
    /// 名称
    #[from(0,~.name.to_string())]
    pub name: String,
    /// 扩展名
    #[from(0,~.ext.to_string())]
    pub ext: String,
    /// 创建者ID
    #[from(0,~.creator_id.to_string())]
    pub creator_id: String,
    /// 创建时间戳
    #[from(0,~.create_timestamp.to_string())]
    pub create_timestamp: String,
    /// 更新者ID
    #[from(0,~.updator_id.to_string())]
    pub updator_id: String,
    /// 更新时间戳
    #[from(0,~.update_timestamp.to_string())]
    pub update_timestamp: String,

    /// 对象存储桶
    #[from(1,OssBucketVo::from(~.clone()))]
    pub oss_bucket: OssBucketVo,
    /// 对象
    #[from(2,OssObjVo::from(~.clone()))]
    pub oss_obj: OssObjVo,
}
