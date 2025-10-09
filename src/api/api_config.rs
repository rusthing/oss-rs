use crate::api::{oss_bucket_api, oss_obj_ref_api};
use crate::settings::SETTINGS;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    let oss_config = SETTINGS.get().unwrap().oss.clone();
    let total_limit = oss_config.upload_file_limit_size.as_u64() as usize;
    cfg.service(
        web::scope("/oss/obj-ref")
            .app_data(MultipartFormConfig::default().total_limit(total_limit)) // 限制文件大小
            .service(oss_obj_ref_api::get_by_id) // 根据id获取实体
            .service(oss_obj_ref_api::upload) // 上传
            .service(oss_obj_ref_api::download) // 下载
            .service(oss_obj_ref_api::preview) // 预览
            .service(oss_obj_ref_api::remove) // 删除
            .service(oss_bucket_api::get_by_id), // 根据id查询
    );
    cfg.service(
        web::scope("/oss/bucket")
            .service(oss_bucket_api::get_by_id) // 根据id获取实体
            .service(oss_bucket_api::add), // 添加
    );
}
