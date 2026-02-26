use crate::web::ctrl::oss_bucket_ctrl::{
    // __path_add, __path_del, __path_del_cascade, __path_modify, __path_save,
    __path_del_cascade,
    __path_get_by_id,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
// #[openapi(paths(add, modify, save, del, del_cascade, get_by_id))]
#[openapi(paths(del_cascade, get_by_id))]
pub struct OssBucketApiDoc;
