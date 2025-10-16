use crate::api::oss_file_api::{__path_download, __path_preview, __path_upload};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(upload, download, preview))]
pub struct OssFileApiDoc;
