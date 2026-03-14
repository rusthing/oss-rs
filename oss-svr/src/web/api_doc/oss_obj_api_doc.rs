use crate::web::ctrl::oss_obj_ctrl::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add, modify, save, del, get_by_id))]
pub struct OssObjApiDoc;
