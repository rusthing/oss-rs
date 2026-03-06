use crate::model::oss_bucket::ActiveModel;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;

#[add_dto]
pub struct OssBucketAddDto {
    /// 名称
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(ActiveValue::Set(~.unwrap()))]
    pub name: Option<String>,
    /// 备注
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    pub remark: Option<Option<String>>,
}

#[modify_dto]
pub struct OssBucketModifyDto {
    /// 名称
    #[into(ActiveValue::Set(~.unwrap()))]
    pub name: Option<String>,
    /// 备注
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    pub remark: Option<Option<String>>,
}

#[save_dto]
pub struct OssBucketSaveDto {
    /// 名称
    pub name: Option<String>,
    /// 备注
    pub remark: Option<Option<String>>,
}
