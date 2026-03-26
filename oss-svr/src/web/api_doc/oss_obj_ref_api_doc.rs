use crate::web::ctrl::oss_obj_ref_ctrl::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add, modify, save, del_by_id, get_by_id, get_by_query_dto))]
pub struct OssObjRefApiDoc;
