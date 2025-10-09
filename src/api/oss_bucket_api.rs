use crate::api::api_error::ApiError;
use crate::cst::user_id_cst::USER_ID_HEADER_NAME;
use crate::svc::oss_bucket_svc;
use crate::to::oss_bucket::OssBucketAddTo;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;

/// 根据id获取桶
#[get("/get-by-id")]
pub async fn get_by_id(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => {
                return Err(ApiError::ValidationError("参数传值不正确{id}".to_string()));
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
#[post("/add")]
pub async fn add(
    json_body: web::Json<OssBucketAddTo>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let mut bucket = json_body.into_inner();

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    bucket.current_user_id = req
        .headers()
        .get(USER_ID_HEADER_NAME)
        .ok_or_else(|| ApiError::ValidationError(format!("缺少必要参数{}", USER_ID_HEADER_NAME)))?
        .to_str()
        .unwrap()
        .to_string()
        .parse::<u64>()
        .map_err(|_| ApiError::ValidationError(format!("参数{}格式不正确", USER_ID_HEADER_NAME)))?;

    let result = oss_bucket_svc::add(bucket).await?;
    Ok(HttpResponse::Ok().json(result))
}
