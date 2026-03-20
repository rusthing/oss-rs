use crate::web::oss_file_ctrl::*;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use linkme::distributed_slice;
use robotech::web::INIT_ROUTERS;

#[distributed_slice(INIT_ROUTERS)]
static INIT_ROUTERS_FN: fn() -> Router = init_routes;

fn init_routes() -> Router {
    Router::new()
        .route(
            "/oss/file/upload/{bucket}",
            post(upload).layer(DefaultBodyLimit::disable()),
        ) // 上传文件
        .route("/oss/file/download/{obj_id}", get(download)) // 下载文件
        .route("/oss/file/preview/{obj_id}", get(preview))
}
