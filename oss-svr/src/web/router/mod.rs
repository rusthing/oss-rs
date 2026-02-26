use axum::Router;

mod oss_bucket_router;

pub fn register() -> Router {
    let router = Router::new();
    oss_bucket_router::routes(router)
}
