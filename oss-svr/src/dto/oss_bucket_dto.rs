use crate::model::oss_bucket::ActiveModel;
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
pub struct OssBucketAddDto {
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
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
#[serde_as]
#[into(ActiveModel)]
#[ghosts(
    creator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssBucketModifyDto {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(match ~ {Some(value)=>ActiveValue::Set(value as i64),None=>ActiveValue::NotSet})]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
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
#[serde_as]
#[into(OssBucketAddDto)]
#[into(OssBucketModifyDto)]
pub struct OssBucketSaveDto {
    // #[into(~ )]
    #[serde_as(as = "Option<String>")]
    pub id: Option<u64>,
    #[into(~.clone())]
    pub name: Option<String>,
    #[into(~.clone())]
    pub remark: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
