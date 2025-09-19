use crate::api::api_error::ApiError;
use crate::app_data::db_app_data::DbAppData;
use crate::svc::oss_obj_svc;
use crate::utils::upload::UploadForm;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use regex::Regex;
use std::collections::HashMap;

#[get("/obj/get-by-id")]
pub async fn get_by_id(
    data: web::Data<DbAppData>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => {
                return Err(ApiError::ValidationError(
                    "以下参数传值不正确{id}".to_string(),
                ));
            }
        },
        None => {
            return Err(ApiError::ValidationError("缺少以下参数{id}".to_string()));
        }
    };
    let ro = oss_obj_svc::get_by_id(&data.db, id).await?;
    Ok(HttpResponse::Ok().json(ro))
}

#[post("/obj/upload/{bucket}")]
pub async fn upload(
    data: web::Data<DbAppData>,
    bucket: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, ApiError> {
    let bucket = bucket.into_inner();
    if bucket.is_empty() {
        return Err(ApiError::ValidationError("无效的bucket".to_string()));
    }

    let file_name = form.file.file_name.unwrap();
    let file_size = form.file.size;
    let temp_file = form.file.file;
    let hash: Option<String> = form.hash.map(|t| t.into_inner());

    // 保存文件到指定的bucket中
    let ro = oss_obj_svc::upload(&data.db, &bucket, &file_name, file_size, hash, temp_file).await?;

    Ok(HttpResponse::Ok().json(ro))
}

#[get("/obj/download/{obj_id}")]
pub async fn download(
    data: web::Data<DbAppData>,
    obj_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let obj_id = obj_id.into_inner();
    // 如果obj_id有后缀，获取后缀并去掉
    // 判断obj_id是否是19位数字加上点再加上任意字符
    let regex = Regex::new(r"^(\d+)\.?([a-zA-Z0-9]*)$").unwrap();
    let (obj_id, ext) = if let Some(captures) = regex.captures(&obj_id) {
        // 匹配成功，提取19位数字和后缀
        let obj_id = captures.get(1).unwrap().as_str().to_string();
        let ext = captures.get(2).unwrap().as_str().to_string();
        (obj_id, Some(ext))
    } else {
        return Err(ApiError::ValidationError("无效的ID".to_string()));
    };

    let (file_name, content) =
        oss_obj_svc::download(&data.db, obj_id.parse::<u64>().unwrap(), ext.unwrap()).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_name),
        ))
        .body(content))
}
