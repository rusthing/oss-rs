use crate::web::ctrl::oss_file_ctrl::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
// #[openapi(paths(upload, download, preview))]
#[openapi(paths(upload))]
pub struct OssFileApiDoc;
