use crate::api::oss_obj_api::{get_by_id, upload};
use crate::config::CONFIG;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    let oss_config = CONFIG.get().unwrap().oss.clone();
    let total_limit = oss_config.upload_file_limit_size.as_u64() as usize;
    cfg.service(
        web::scope("/oss")
            .app_data(MultipartFormConfig::default().total_limit(total_limit))
            .service(get_by_id) // 根据id查询
            .service(upload), // 上传
    );
}
