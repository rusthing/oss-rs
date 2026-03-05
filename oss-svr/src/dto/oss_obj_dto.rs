use crate::model::oss_obj::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssObjAddDto {
    #[validate(
        required(message = "路径不能为空"),
        length(min = 1, message = "路径不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap_or("".to_string())))]
    pub path: Option<String>,
    #[validate(required(message = "文件大小不能为空"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub size: Option<u64>,
    #[validate(
        required(message = "Hash不能为空"),
        length(min = 1, message = "Hash不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
}

#[modify_dto]
pub struct OssObjModifyDto {
    #[into(ActiveValue::Set(~.unwrap()))]
    pub path: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub size: Option<u64>,
    #[into(ActiveValue::Set(~.unwrap()))]
    pub hash: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
}

#[save_dto]
pub struct OssObjSaveDto {
    pub path: Option<String>,
    pub size: Option<u64>,
    pub hash: Option<String>,
    pub is_completed: Option<bool>,
}
