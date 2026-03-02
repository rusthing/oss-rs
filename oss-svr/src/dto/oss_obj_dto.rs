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
    #[into(ActiveValue::Set(~.clone().unwrap_or("".to_string())))]
    pub path: Option<String>,
    #[validate(
        required(message = "文件大小不能为空"),
        length(min = 1, message = "文件大小不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub size: Option<String>,
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
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub path: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub size: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
}

#[save_dto]
pub struct OssObjSaveDto {
    #[into(~.clone())]
    pub path: Option<String>,
    #[into(~.clone())]
    pub size: Option<String>,
    #[into(~.clone())]
    pub hash: Option<String>,
    pub is_completed: Option<bool>,
}
