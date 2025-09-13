use crate::api::oss_obj_api::get_by_id;
use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/oss").service(get_by_id));
}
