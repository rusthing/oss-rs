use crate::model::oss_bucket::ActiveModel;
use derive_setters::Setters;
use robotech_macros::{add_dto, modify_dto, save_dto};
use sea_orm::ActiveValue;
use serde_with::serde_as;
use typed_builder::TypedBuilder;
use wheel_rs::serde::option_option_serde;

#[add_dto]
pub struct OssBucketAddDto {
    /// 名称
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// 备注
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    #[builder(default, setter(strip_option))]
    #[serde(deserialize_with = "option_option_serde::deserialize")]
    pub remark: Option<Option<String>>,
}

#[modify_dto]
pub struct OssBucketModifyDto {
    /// 名称
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    pub name: Option<String>,
    /// 备注
    #[into(match ~ {Some(v)=>ActiveValue::Set(v),None=>ActiveValue::NotSet})]
    #[builder(default, setter(strip_option))]
    #[serde(deserialize_with = "option_option_serde::deserialize")]
    pub remark: Option<Option<String>>,
}

#[save_dto]
pub struct OssBucketSaveDto {
    /// 名称
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// 备注
    #[builder(default, setter(strip_option))]
    #[serde(deserialize_with = "option_option_serde::deserialize")]
    pub remark: Option<Option<String>>,
}
