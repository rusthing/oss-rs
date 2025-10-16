use crate::api::oss_obj_ref_api::{
    __path_add, __path_del, __path_get_by_id, __path_modify, __path_save,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add, modify, save, del, get_by_id))]
pub struct OssObjRefApiDoc;
