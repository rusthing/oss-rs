use crate::dto::oss_bucket_dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::svc::oss_bucket_svc::OssBucketSvc;
use crate::vo::oss_bucket_vo::OssBucketVo;
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
struct OssBucketCtrl;

/// # 级联删除记录
///
/// 该接口用于级联删除一个已存在的记录及其关联数据
///
/// ## 请求参数
/// * `id` - 待删除记录的唯一标识符，类型为u64
///
/// ## 错误处理
/// * 当缺少参数`id`时，返回`ValidationError`错误
/// * 当参数`id`格式不正确时，返回`ValidationError`错误
/// * 当根据ID找不到对应记录时，返回相应的错误信息
#[utoipa::path(
    path = "/oss/bucket/cascade",
    params(
        ("id", description = "记录的唯一标识符，类型为u64")
    ),
    responses((status = OK, body = Ro<String>))
)]
#[delete("/cascade")]
#[log_call]
pub async fn del_cascade(
    query: Query<HashMap<String, String>>,
    req: HttpRequest,
) -> Result<HttpResponse, CtrlError> {
    let id = get_id_from_query_params(query)?;

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(req)?;
    Ok(HttpResponse::Ok()
        .json(OssBucketSvc::del_cascade::<DatabaseTransaction>(id, current_user_id, None).await?))
}
