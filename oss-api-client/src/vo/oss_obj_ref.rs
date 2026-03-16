use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none] // 忽略空字段(好像必须放在#[derive(o2o, Serialize)]的上方才能起效)
#[derive(ToSchema, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OssObjRefVo {
    /// ID
    pub id: u64,
    /// 对象ID
    pub obj_id: u64,
    /// 存储桶ID
    pub bucket_id: u64,
    /// 名称
    pub name: String,
    /// 文件扩展名
    pub ext: Option<String>,
    /// 下载URL
    pub download_url: String,
    /// 预览URL
    pub preview_url: Option<String>,
    /// 创建者ID
    pub creator_id: u64,
    /// 创建时间戳
    pub create_timestamp: u64,
    /// 更新者ID
    pub updator_id: u64,
    /// 更新时间戳
    pub update_timestamp: u64,
}
