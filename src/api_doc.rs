use crate::api::oss_bucket_api::__path_get_by_id;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_by_id))]
pub struct ApiDoc;
