use crate::dto::oss_obj_ref_dto::{OssObjRefAddDto, OssObjRefModifyDto, OssObjRefSaveDto};
use crate::svc::oss_obj_ref_svc::OssObjRefSvc;
use crate::vo::oss_obj_ref_vo::OssObjRefVo;
use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, Result, delete, get, post, put, web};
use robotech::macros::log_call;
use robotech::ro::Ro;
use robotech::web::CtrlError;
use robotech_macros::ctrl;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::collections::HashMap;
use validator::Validate;

// #[ctrl]
struct OssObjRefCtrl;
