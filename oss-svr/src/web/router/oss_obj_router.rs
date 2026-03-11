use crate::web::ctrl::oss_obj_ctrl::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub(super) fn routes(router: Router) -> Router {
    router
        .route("/oss/obj", post(add)) // 添加
        .route("/oss/obj", put(modify)) // 修改
        .route("/oss/obj/save", post(save)) // 保存
        .route("/oss/obj/{id}", delete(del)) // 删除
        .route("/oss/obj/{id}", get(get_by_id)) // 根据id获取
}
