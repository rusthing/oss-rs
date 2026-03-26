use crate::web::ctrl::oss_file_ctrl::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(upload, download, preview))]
pub struct OssFileApiDoc;
