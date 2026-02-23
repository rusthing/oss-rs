use crate::dto::oss_obj_dto::{OssObjAddDto, OssObjModifyDto, OssObjSaveDto};
use crate::svc::oss_obj_svc::OssObjSvc;
use crate::vo::oss_obj_vo::OssObjVo;
use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, Result, delete, get, post, put, web};
use robotech::macros::log_call;
use robotech::ro::Ro;
use robotech::web::CtrlError;
use robotech::web::ctrl_utils::{get_current_user_id, get_id_from_query_params};
use robotech_macros::ctrl;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::collections::HashMap;
use tracing::instrument;
use validator::Validate;

#[ctrl]
struct OssObjCtrl;
