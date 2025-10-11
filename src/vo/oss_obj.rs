use crate::model::oss_obj::Model;
use o2o::o2o;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::string::ToString;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(o2o, Debug, Serialize)]
#[from(Model)]
#[serde(rename_all = "camelCase")]
pub struct OssObjVo {
    /// ID
    #[from(~.to_string())]
    pub id: String,
    /// 文件Hash
    #[from(~.to_string())]
    pub hash: String,
    /// 文件大小
    #[from(~.to_string())]
    pub size: String,
    /// 访问URL地址
    #[from(~.to_string())]
    pub url: String,
    /// 是否完成
    pub is_completed: bool,
    /// 创建者ID
    #[from(~.to_string())]
    pub creator_id: String,
    /// 创建时间戳
    #[from(~.to_string())]
    pub create_timestamp: String,
    /// 更新者ID
    #[from(~.to_string())]
    pub updator_id: String,
    /// 更新时间戳
    #[from(~.to_string())]
    pub update_timestamp: String,
}
