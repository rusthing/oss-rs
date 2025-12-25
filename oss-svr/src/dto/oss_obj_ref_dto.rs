use crate::model::oss_obj_ref::ActiveModel;
use o2o::o2o;
use sea_orm::ActiveValue;
use serde::Deserialize;
use serde_with::serde_as;
use utoipa::ToSchema;
use validator::Validate;

#[derive(o2o, ToSchema, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[serde_as]
#[into(ActiveModel)]
#[ghosts(
    updator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssObjRefAddDto {
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    #[validate(required(message = "对象ID不能为空"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
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
    #[serde(skip_deserializing)]
    #[into(creator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[serde_as]
#[into(ActiveModel)]
#[ghosts(
    creator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssObjRefModifyDto {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub ext: Option<String>,
    #[serde(skip_deserializing)]
    #[into(updator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
#[into(OssObjRefAddDto)]
#[into(OssObjRefModifyDto)]
pub struct OssObjRefSaveDto {
    // #[into(~ )]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    // #[into(~ )]
    #[serde_as(as = "Option<String>")]
    pub obj_id: Option<u64>,
    // #[into(~ )]
    #[serde_as(as = "Option<String>")]
    pub bucket_id: Option<u64>,
    #[into(~.clone())]
    pub name: Option<String>,
    #[into(~.clone())]
    pub ext: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
