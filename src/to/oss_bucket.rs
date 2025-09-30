use o2o::o2o;
use serde::Deserialize;

use crate::model::oss_bucket::Entity;
use validator::Validate;

#[derive(o2o, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[to(Entity)]
pub struct OssBucketAddTo {
    #[validate(length(min = 1, message = "名称不能为空"))]
    pub name: String,
}
