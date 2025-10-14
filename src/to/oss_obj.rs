use crate::model::oss_obj::ActiveModel;
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
pub struct OssObjAddTo {
    #[into(match ~.clone() {Some(value)=>ActiveValue::Set(value.parse::<i64>().unwrap()),None=>ActiveValue::NotSet})]
    pub id: Option<String>,
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
    #[validate(
        required(message = "Url不能为空"),
        length(min = 1, message = "Url不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
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
pub struct OssObjModifyTo {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub id: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub path: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub size: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
    #[into(match ~ {Some(value)=>ActiveValue::Set(value),None=>ActiveValue::NotSet})]
    pub is_completed: Option<bool>,
    #[serde(skip_deserializing)]
    #[into(updator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[into(OssObjAddTo)]
#[into(OssObjModifyTo)]
pub struct OssObjSaveTo {
    #[into(~.clone())]
    pub id: Option<String>,
    #[into(~.clone())]
    pub path: Option<String>,
    #[into(~.clone())]
    pub size: Option<String>,
    #[into(~.clone())]
    pub hash: Option<String>,
    #[into(~.clone())]
    pub url: Option<String>,
    pub is_completed: Option<bool>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
