use crate::api::api_error::ApiError;
use crate::svc::oss_bucket_svc;
use crate::to::oss_bucket::OssBucketAddTo;
use actix_web::{get, post, web, HttpResponse, Result};
use std::collections::HashMap;

/// 根据id获取桶
#[get("/bucket/get-by-id")]
pub async fn get_by_id(
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
            return Err(ApiError::ValidationError("缺少必要参数{id}".to_string()));
        }
    };
    let ro = oss_bucket_svc::get_by_id(id).await?;
    Ok(HttpResponse::Ok().json(ro))
}

/// 添加
#[post("/bucket/add")]
pub async fn add(form: web::Json<OssBucketAddTo>) -> Result<HttpResponse, ApiError> {
    let bucket = OssBucketAddTo::from(form.into_inner());
    let result = oss_bucket_svc::add(bucket).await?;
    Ok(HttpResponse::Ok().json(result))
}
