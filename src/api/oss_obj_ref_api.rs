use crate::utils::api_utils::ApiError;
use crate::svc::oss_obj_ref_svc;
use actix_web::{delete, get, web, HttpResponse, Result};
use std::collections::HashMap;

/// 根据id获取对象引用
#[get("/get-by-id")]
pub async fn get_by_id(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => {
                return Err(ApiError::ValidationError(format!(
                    "参数<id>格式不正确: {}",
                    id_str
                )));
            }
        },
        None => {
            return Err(ApiError::ValidationError("缺少必要参数<id>".to_string()));
        }
    };
    let ro = oss_obj_ref_svc::get_by_id(id).await?;
    Ok(HttpResponse::Ok().json(ro))
}

/// 移除对象引用
#[delete("/remove")]
pub async fn remove(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, ApiError> {
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
    let ro = oss_obj_ref_svc::remove(id).await?;
    Ok(HttpResponse::Ok().json(ro))
}
