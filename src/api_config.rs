use crate::api::oss_bucket_api_doc::OssBucketApiDoc;
use crate::api::oss_file_api_doc::OssFileApiDoc;
use crate::api::oss_obj_api_doc::OssObjApiDoc;
use crate::api::{oss_bucket_api, oss_file_api, oss_obj_api, oss_obj_ref_api};
use crate::settings::SETTINGS;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

/// 初始化api配置
pub fn init_api_config(cfg: &mut web::ServiceConfig) {
    let oss_config = SETTINGS.get().unwrap().oss.clone();
    let total_limit = oss_config.upload_file_limit_size.as_u64() as usize;
    cfg.service(
        web::scope("/oss/bucket")
            .service(oss_bucket_api::add) // 添加
            .service(oss_bucket_api::save) // 根据id获取实体
            .service(oss_bucket_api::modify) // 根据id获取实体
            .service(oss_bucket_api::del) // 删除
            .service(oss_bucket_api::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/obj")
            .service(oss_obj_api::add) // 添加
            .service(oss_obj_api::save) // 根据id获取实体
            .service(oss_obj_api::modify) // 根据id获取实体
            .service(oss_obj_api::del) // 删除
            .service(oss_obj_api::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/obj-ref").service(oss_obj_ref_api::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/file")
            .app_data(MultipartFormConfig::default().total_limit(total_limit)) // 限制文件大小
            .service(oss_file_api::del) // 删除文件
            .service(oss_file_api::upload) // 上传文件
            .service(oss_file_api::download) // 下载文件
            .service(oss_file_api::preview), // 预览
    );
    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("桶", "/api-docs/bucket-openapi.json"),
            OssBucketApiDoc::openapi(),
        ),
        (
            Url::new("对象", "/api-docs/obj-openapi.json"),
            OssObjApiDoc::openapi(),
        ),
        (
            Url::new("文件", "/api-docs/file-openapi.json"),
            OssFileApiDoc::openapi(),
        ),
    ]));
}
