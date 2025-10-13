use crate::svc::oss_obj_ref_svc;
use crate::utils::api_utils::ApiError;
use actix_web::{get, web, HttpResponse, Result};
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
                let msg = format!("参数<id>格式错误: {}", id_str);
                return Err(ApiError::from(validator::ValidationError::new(Box::leak(
                    msg.into_boxed_str(),
                ))));
            }
        },
        None => {
            return Err(ApiError::from(validator::ValidationError::new(
                "缺少必要参数<id>",
            )));
        }
    };

    let ro = oss_obj_ref_svc::get_by_id(id).await?;
    Ok(HttpResponse::Ok().json(ro))
}
