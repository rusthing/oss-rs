use crate::model::oss_obj_ref::ActiveModel;
use o2o::o2o;
use sea_orm::ActiveValue;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(o2o, ToSchema, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[into(ActiveModel)]
#[ghosts(
    updator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssObjRefAddDto {
    #[into(match ~.clone() {Some(value)=>ActiveValue::Set(value.parse::<i64>().unwrap()),None=>ActiveValue::NotSet})]
    pub id: Option<String>,
    #[validate(
        required(message = "对象ID不能为空"),
        length(min = 1, message = "对象ID不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub obj_id: Option<String>,
    #[validate(
        required(message = "对象ID不能为空"),
        length(min = 1, message = "桶ID不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub bucket_id: Option<String>,
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
#[into(ActiveModel)]
#[ghosts(
    creator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssObjRefModifyDto {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub id: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub obj_id: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub bucket_id: Option<String>,
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
#[into(OssObjRefAddDto)]
#[into(OssObjRefModifyDto)]
pub struct OssObjRefSaveDto {
    #[into(~.clone())]
    pub id: Option<String>,
    #[into(~.clone())]
    pub obj_id: Option<String>,
    #[into(~.clone())]
    pub bucket_id: Option<String>,
    #[into(~.clone())]
    pub name: Option<String>,
    #[into(~.clone())]
    pub ext: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
