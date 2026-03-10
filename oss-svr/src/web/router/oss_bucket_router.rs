use crate::web::ctrl::oss_bucket_ctrl::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub(super) fn routes(router: Router) -> Router {
    router
        .route("/oss/bucket", post(add)) // 添加
        .route("/oss/bucket", put(modify)) // 修改
        .route("/oss/bucket/save", post(save)) // 保存
        .route("/oss/bucket/{id}", delete(del)) // 删除
        .route("/oss/bucket/{id}", get(get_by_id)) // 根据id获取
        .route("/oss/bucket/cascade/{id}", delete(del_cascade)) // 级联删除
}
