use crate::web::ctrl::oss_bucket_ctrl::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add, modify, save, del, del_cascade, get_by_id))]
pub struct OssBucketApiDoc;
