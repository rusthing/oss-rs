use crate::app_data::db_app_data::DbAppData;
use actix_web::{get, web, HttpResponse, Result};

#[get("/obj/get-by-id")]
pub async fn get_by_id(data: web::Data<DbAppData>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
