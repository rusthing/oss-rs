use crate::model::oss_obj_ref::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssObjRefAddDto {
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[validate(
        required(message = "文件扩展名不能为空"),
        length(min = 1, message = "文件扩展名不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub ext: Option<String>,
    #[validate(
        required(message = "Url不能为空"),
        length(min = 1, message = "Url不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
}

#[modify_dto]
pub struct OssObjRefModifyDto {
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub ext: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
}

#[save_dto]
pub struct OssObjRefSaveDto {
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    pub name: Option<String>,
    pub ext: Option<String>,
    pub url: Option<String>,
}
