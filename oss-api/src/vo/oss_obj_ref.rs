use crate::vo::oss_bucket::OssBucketVo;
use crate::vo::oss_obj::OssObjVo;
use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(ToSchema, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OssObjRefVo {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 扩展名
    pub ext: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间戳
    pub create_timestamp: String,
    /// 更新者ID
    pub updator_id: String,
    /// 更新时间戳
    pub update_timestamp: String,

    /// 对象存储桶
    pub oss_bucket: OssBucketVo,
    /// 对象
    pub oss_obj: OssObjVo,
}
