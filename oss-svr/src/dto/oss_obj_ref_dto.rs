use crate::model::oss_obj_ref::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssObjRefAddDto {
    /// 对象ID
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    /// 存储桶ID
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    /// 名称
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub name: Option<String>,
    /// 文件扩展名
    #[validate(
        required(message = "文件扩展名不能为空"),
        length(min = 1, message = "文件扩展名不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub ext: Option<String>,
    /// Url
    #[validate(
        required(message = "Url不能为空"),
        length(min = 1, message = "Url不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub url: Option<String>,
}

#[modify_dto]
pub struct OssObjRefModifyDto {
    /// 对象ID
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    /// 存储桶ID
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    /// 名称
    #[into(ActiveValue::Set(~.unwrap()))]
    pub name: Option<String>,
    /// 文件扩展名
    #[into(ActiveValue::Set(~.unwrap()))]
    pub ext: Option<String>,
    /// Url
    #[into(ActiveValue::Set(~.unwrap()))]
    pub url: Option<String>,
}

#[save_dto]
pub struct OssObjRefSaveDto {
    /// 对象ID
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    /// 存储桶ID
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    /// 名称
    pub name: Option<String>,
    /// 文件扩展名
    pub ext: Option<String>,
    /// Url
    pub url: Option<String>,
}
