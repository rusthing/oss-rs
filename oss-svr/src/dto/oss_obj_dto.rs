use crate::model::oss_obj::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssObjAddDto {
    /// 路径
    #[validate(
        required(message = "路径不能为空"),
        length(min = 1, message = "路径不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub path: Option<String>,
    /// 文件大小
    #[validate(required(message = "文件大小不能为空"))]
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub size: Option<u64>,
    /// Hash
    #[validate(
        required(message = "Hash不能为空"),
        length(min = 1, message = "Hash不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub hash: Option<String>,
    /// 是否完成
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
}

#[modify_dto]
pub struct OssObjModifyDto {
    /// 路径
    #[into(ActiveValue::Set(~.unwrap()))]
    pub path: Option<String>,
    /// 文件大小
    #[into(match ~ {Some(v)=>ActiveValue::Set(v as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub size: Option<u64>,
    /// Hash
    #[into(ActiveValue::Set(~.unwrap()))]
    pub hash: Option<String>,
    /// 是否完成
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
}

#[save_dto]
pub struct OssObjSaveDto {
    /// 路径
    pub path: Option<String>,
    /// 文件大小
    pub size: Option<u64>,
    /// Hash
    pub hash: Option<String>,
    /// 是否完成
    pub is_completed: Option<bool>,
}
