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
    id: Default::default(),
    is_completed: Default::default(),
    updator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssObjAddTo {
    #[validate(
        required(message = "路径不能为空"),
        length(min = 1, message = "路径不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap_or("".to_string())))]
    pub path: Option<String>,
    #[validate(required(message = "文件大小不能为空"))]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub size: Option<i64>,
    #[validate(required(message = "Hash不能为空"))]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[validate(required(message = "Url不能为空"))]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
    #[serde(skip_deserializing)]
    #[into(creator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[into(ActiveModel)]
#[ghosts(
    is_completed: Default::default(),
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
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub size: Option<i64>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub hash: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub url: Option<String>,
    #[serde(skip_deserializing)]
    #[into(updator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[into(OssObjAddTo)]
#[into(OssObjModifyTo)]
pub struct OssObjSaveTo {
    #[into(OssObjModifyTo| ~.clone())]
    #[ghost(OssObjAddTo)]
    pub id: Option<String>,
    #[into(~.clone())]
    pub path: Option<String>,
    #[into(~.clone())]
    pub size: Option<i64>,
    #[into(~.clone())]
    pub hash: Option<String>,
    #[into(~.clone())]
    pub url: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
