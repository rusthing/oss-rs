use o2o::o2o;
use sea_orm::ActiveValue;
use serde::Deserialize;

use crate::model::oss_bucket::ActiveModel;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OssBucketAddTo {
    #[validate(required(message = "名称不能为空"))]
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}

#[allow(clippy::from_over_into)]
impl Into<ActiveModel> for OssBucketAddTo {
    fn into(self) -> ActiveModel {
        ActiveModel {
            name: ActiveValue::set(self.name.unwrap()),
            creator_id: ActiveValue::set(self.current_user_id as i64),
            updator_id: ActiveValue::set(self.current_user_id as i64),
            ..Default::default()
        }
    }
}

#[derive(o2o, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[into(ActiveModel)]
#[ghosts(
    creator_id: Default::default(),
    create_timestamp: Default::default(),
    update_timestamp: Default::default(),
)]
pub struct OssBucketModifyTo {
    #[validate(required(message = "缺少必要参数<id>"))]
    #[into(ActiveValue::Set(~.clone().unwrap().parse::<i64>().unwrap()))]
    pub id: Option<String>,
    #[into(ActiveValue::Set(~.clone().unwrap()))]
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    // #[map(creator_id, ActiveValue::Set(~ as i64), updator_id, ActiveValue::Set(~ as i64))]
    // #[into(creator_id,ActiveValue::Set(~ as i64))]
    #[into(updator_id,ActiveValue::Set(~ as i64))]
    pub current_user_id: u64,
}

#[derive(o2o, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[into(OssBucketAddTo)]
#[into(OssBucketModifyTo)]
pub struct OssBucketSaveTo {
    #[into(OssBucketModifyTo| ~.clone())]
    #[ghost(OssBucketAddTo)]
    pub id: Option<String>,
    #[into(~.clone())]
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}
