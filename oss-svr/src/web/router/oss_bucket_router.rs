use crate::web::ctrl::oss_bucket_ctrl;
use axum::{
    routing::{delete, get},
    Router,
};

pub(super) fn routes(router: Router) -> Router {
    router
        .route("/oss/bucket/{id}", get(oss_bucket_ctrl::get_by_id))
        .route("/oss/bucket/cascade/{id}", delete(oss_bucket_ctrl::del_cascade))
}
