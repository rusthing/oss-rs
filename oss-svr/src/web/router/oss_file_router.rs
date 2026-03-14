use crate::web::oss_file_ctrl::*;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

pub(super) fn routes(router: Router) -> Router {
    router.route(
        "/oss/file/upload/{bucket}",
        post(upload).layer(DefaultBodyLimit::disable()),
    ) // 上传文件
}
