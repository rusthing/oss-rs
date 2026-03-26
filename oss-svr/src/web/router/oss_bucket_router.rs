use crate::web::ctrl::oss_bucket_ctrl::*;
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
        .route("/oss/bucket", post(add)) // 添加
        .route("/oss/bucket", put(modify)) // 修改
        .route("/oss/bucket/save", post(save)) // 保存
        .route("/oss/bucket/{id}", delete(del_by_id)) // 根据id删除
        .route("/oss/bucket/{id}", get(get_by_id)) // 根据id获取
        .route("/oss/bucket", get(get_by_query_dto)) // 根据查询条件获取
        .route("/oss/bucket/cascade/{id}", delete(del_cascade)) // 级联删除
}
