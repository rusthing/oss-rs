use crate::model::oss_obj::ActiveModel;
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
pub struct OssObjAddDto {
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
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
pub struct OssObjModifyDto {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub path: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub size: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
    #[serde(skip_deserializing)]
    #[into(updator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
#[into(OssObjAddDto)]
#[into(OssObjModifyDto)]
pub struct OssObjSaveDto {
    // #[into(~ )]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    #[into(~.clone())]
    pub path: Option<String>,
    #[into(~.clone())]
    pub size: Option<String>,
    #[into(~.clone())]
    pub hash: Option<String>,
    pub is_completed: Option<bool>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
