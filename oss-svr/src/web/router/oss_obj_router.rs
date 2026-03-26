use crate::web::ctrl::oss_obj_ctrl::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use linkme::distributed_slice;
use robotech::web::INIT_ROUTERS_SLICE;

#[distributed_slice(INIT_ROUTERS_SLICE)]
static INIT_ROUTERS_FN: fn() -> Router = init_routes;

fn init_routes() -> Router {
    Router::new()
        .route("/oss/obj", post(add)) // 添加
        .route("/oss/obj", put(modify)) // 修改
        .route("/oss/obj/save", post(save)) // 保存
        .route("/oss/obj/{id}", delete(del_by_id)) // 删除
        .route("/oss/obj/{id}", get(get_by_id)) // 根据id获取
        .route("/oss/obj", get(get_by_query_dto)) // 根据查询条件获取
}
