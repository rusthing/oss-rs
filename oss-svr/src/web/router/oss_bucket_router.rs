use crate::web::ctrl::oss_bucket_ctrl;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub(super) fn routes(router: Router) -> Router {
    router
        .route("/oss/bucket", post(oss_bucket_ctrl::add))
        .route("/oss/bucket", put(oss_bucket_ctrl::modify))
        .route("/oss/bucket/save", post(oss_bucket_ctrl::save))
        .route("/oss/bucket/{id}", delete(oss_bucket_ctrl::del))
        .route("/oss/bucket/{id}", get(oss_bucket_ctrl::get_by_id))
        .route(
            "/oss/bucket/cascade/{id}",
            delete(oss_bucket_ctrl::del_cascade),
        )
}
