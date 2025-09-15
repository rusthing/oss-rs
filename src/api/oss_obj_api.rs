use crate::app_data::db_app_data::DbAppData;
use crate::svc::oss_obj_svc;
use actix_web::{get, web, HttpResponse, Result};
use std::collections::HashMap;

#[get("/obj/get-by-id")]
pub async fn get_by_id(
    data: web::Data<DbAppData>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid id parameter")),
        },
        None => return Ok(HttpResponse::BadRequest().body("Missing id parameter")),
    };
    Ok(HttpResponse::Ok().json(oss_obj_svc::get_by_id(&data.db, id).await))
}
