use crate::api::oss_obj_ref_api::{download, get_by_id, preview, remove, upload};
use crate::settings::SETTINGS;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    let oss_config = SETTINGS.get().unwrap().oss.clone();
    let total_limit = oss_config.upload_file_limit_size.as_u64() as usize;
    cfg.service(
        web::scope("/oss")
            .app_data(MultipartFormConfig::default().total_limit(total_limit))
            .service(get_by_id) // 根据id查询
            .service(upload) // 上传
            .service(download) // 下载
            .service(preview) // 预览
            .service(remove), // 删除
    );
}
