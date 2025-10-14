use crate::api::oss_bucket_api::{
    __path_add, __path_del, __path_del_cascade, __path_get_by_id, __path_modify, __path_save,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add, modify, save, del, del_cascade, get_by_id))]
pub struct OssBucketApiDoc;
