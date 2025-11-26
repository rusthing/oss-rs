use crate::model::oss_bucket::ActiveModel;
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
pub struct OssBucketAddDto {
    #[into(match ~.clone() {Some(value)=>ActiveValue::Set(value.parse::<i64>().unwrap()),None=>ActiveValue::NotSet})]
    pub id: Option<String>,
    #[validate(
        required(message = "名称不能为空"),
        length(min = 1, message = "名称不能为空")
    )]
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone()))]
    pub remark: Option<String>,
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
pub struct OssBucketModifyDto {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub id: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[into(ActiveValue::Set(~.clone()))]
    pub remark: Option<String>,
    #[serde(skip_deserializing)]
    #[into(updator_id, ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[into(OssBucketAddDto)]
#[into(OssBucketModifyDto)]
pub struct OssBucketSaveDto {
    #[into(~.clone())]
    pub id: Option<String>,
    #[into(~.clone())]
    pub name: Option<String>,
    #[into(~.clone())]
    pub remark: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
