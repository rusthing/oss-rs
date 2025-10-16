use crate::ctrl::oss_file_ctrl::{__path_download, __path_preview, __path_upload};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(upload, download, preview))]
pub struct OssFileApiDoc;
