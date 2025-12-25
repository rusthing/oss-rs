use crate::model::{oss_bucket, oss_obj, oss_obj_ref};
use crate::vo::oss_bucket_vo::OssBucketVo;
use crate::vo::oss_obj_vo::OssObjVo;
use o2o::o2o;
use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none};
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, ToSchema, Debug, Serialize, Clone)]
#[from((oss_obj_ref::Model, oss_bucket::Model, oss_obj::Model))]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OssObjRefVo {
    /// ID
    #[from(0,~.id as u64)]
    #[serde_as(as = "String")]
    pub id: u64,
    /// 名称
    #[from(0,~.name.to_string())]
    pub name: String,
    /// 扩展名
    #[from(0,~.ext.to_string())]
    pub ext: String,
    /// 创建者ID
    #[from(0,~.creator_id as u64)]
    #[serde_as(as = "String")]
    pub creator_id: u64,
    /// 创建时间戳
    #[from(0,~.create_timestamp as u64)]
    #[serde_as(as = "String")]
    pub create_timestamp: u64,
    /// 更新者ID
    #[from(0,~.updator_id as u64)]
    #[serde_as(as = "String")]
    pub updator_id: u64,
    /// 更新时间戳
    #[from(0,~.update_timestamp as u64)]
    #[serde_as(as = "String")]
    pub update_timestamp: u64,

    /// 对象存储桶
    #[from(1,OssBucketVo::from(~.clone()))]
    pub oss_bucket: OssBucketVo,
    /// 对象
    #[from(2,OssObjVo::from(~.clone()))]
    pub oss_obj: OssObjVo,
}
