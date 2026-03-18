use crate::web::oss_file_ctrl::*;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;

pub(super) fn routes(router: Router) -> Router {
    router
        .route(
            "/oss/file/upload/{bucket}",
            post(upload).layer(DefaultBodyLimit::disable()),
        ) // 上传文件
        .route("/oss/file/download/{obj_id}", get(download)) // 下载文件
        .route("/oss/file/preview/{obj_id}", get(preview)) // 预览文件
}
