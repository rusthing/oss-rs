use crate::model::oss_bucket::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssBucketAddDto {
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone()))]
    pub remark: Option<String>,
}

#[modify_dto]
pub struct OssBucketModifyDto {
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone()))]
    pub remark: Option<String>,
}

#[save_dto]
pub struct OssBucketSaveDto {
    #[into(~.clone())]
    pub name: Option<String>,
    #[into(~.clone())]
    pub remark: Option<String>,
}
