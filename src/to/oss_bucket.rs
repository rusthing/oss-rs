use sea_orm::ActiveValue;
use serde::Deserialize;

use crate::model::oss_bucket::ActiveModel;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OssBucketAddTo {
    #[validate(length(min = 1, message = "名称不能为空"))]
    pub name: String,
    #[serde(skip_deserializing)]
    pub current_user_id: u64,
}

#[allow(clippy::from_over_into)]
impl Into<ActiveModel> for OssBucketAddTo {
    fn into(self) -> ActiveModel {
        ActiveModel {
            name: ActiveValue::set(self.name),
            creator_id: ActiveValue::set(self.current_user_id as i64),
            updator_id: ActiveValue::set(self.current_user_id as i64),
            ..Default::default()
        }
    }
}
