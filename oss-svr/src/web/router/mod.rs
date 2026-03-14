use axum::Router;

mod oss_bucket_router;
mod oss_file_router;
mod oss_obj_ref_router;
mod oss_obj_router;

pub fn register() -> Router {
    let mut router = Router::new();
    router = oss_bucket_router::routes(router);
    router = oss_obj_router::routes(router);
    router = oss_obj_ref_router::routes(router);
    router = oss_file_router::routes(router);
    router
}
