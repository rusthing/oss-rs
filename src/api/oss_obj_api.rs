use crate::api::api_error::ApiError;
use crate::app_data::db_app_data::DbAppData;
use crate::svc::oss_obj_svc;
use actix_web::{get, web, HttpResponse, Result};
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
    if ro.extra == None {
        return Err(ApiError::NotFound());
    }
    Ok(HttpResponse::Ok().json(ro))
}
