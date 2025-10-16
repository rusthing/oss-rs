use crate::api_doc::oss_bucket_api_doc::OssBucketApiDoc;
use crate::api_doc::oss_file_api_doc::OssFileApiDoc;
use crate::api_doc::oss_obj_api_doc::OssObjApiDoc;
use crate::api_doc::oss_obj_ref_api_doc::OssObjRefApiDoc;
use crate::ctrl::{oss_bucket_ctrl, oss_file_ctrl, oss_obj_ctrl, oss_obj_ref_ctrl};
use crate::settings::SETTINGS;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

/// # 配置WebService
pub fn web_service_config(cfg: &mut web::ServiceConfig) {
    let oss_config = SETTINGS.get().unwrap().oss.clone();
    let total_limit = oss_config.upload_file_limit_size.as_u64() as usize;
    cfg.service(
        web::scope("/oss/bucket")
            .service(oss_bucket_ctrl::add) // 添加
            .service(oss_bucket_ctrl::modify) // 根据id获取实体
            .service(oss_bucket_ctrl::save) // 根据id获取实体
            .service(oss_bucket_ctrl::del) // 删除
            .service(oss_bucket_ctrl::del_cascade) // 级联删除
            .service(oss_bucket_ctrl::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/obj")
            .service(oss_obj_ctrl::add) // 添加
            .service(oss_obj_ctrl::modify) // 根据id获取实体
            .service(oss_obj_ctrl::save) // 根据id获取实体
            .service(oss_obj_ctrl::del) // 删除
            .service(oss_obj_ctrl::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/obj-ref")
            .service(oss_obj_ref_ctrl::add) // 添加
            .service(oss_obj_ref_ctrl::modify) // 根据id获取实体
            .service(oss_obj_ref_ctrl::save) // 根据id获取实体
            .service(oss_obj_ref_ctrl::del) // 删除
            .service(oss_obj_ref_ctrl::get_by_id), // 根据id获取实体
    );
    cfg.service(
        web::scope("/oss/file")
            .app_data(MultipartFormConfig::default().total_limit(total_limit)) // 限制文件大小
            .service(oss_file_ctrl::upload) // 上传文件
            .service(oss_file_ctrl::download) // 下载文件
            .service(oss_file_ctrl::preview), // 预览
    );
    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("桶", "/ctrl-docs/bucket-openapi.json"),
            OssBucketApiDoc::openapi(),
        ),
        (
            Url::new("对象", "/ctrl-docs/obj-openapi.json"),
            OssObjApiDoc::openapi(),
        ),
        (
            Url::new("对象引用", "/ctrl-docs/obj-ref-openapi.json"),
            OssObjRefApiDoc::openapi(),
        ),
        (
            Url::new("文件", "/ctrl-docs/file-openapi.json"),
            OssFileApiDoc::openapi(),
        ),
    ]));
}
