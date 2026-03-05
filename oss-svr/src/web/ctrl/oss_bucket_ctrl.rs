use crate::dto::oss_bucket_dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::model::oss_bucket::Model;
use crate::svc::oss_bucket_svc;
use crate::svc::oss_bucket_svc::OssBucketSvc;
use crate::vo::oss_bucket_vo::OssBucketVo;
use crate::web::ctrl::oss_bucket_ctrl;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::Json;
use axum::routing::{delete, get};
use robotech::macros::log_call;
use robotech::ro::Ro;
use robotech::web::CtrlError;
use robotech::web::ctrl_utils::get_current_user_id;
use robotech_macros::ctrl;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::collections::HashMap;
use validator::Validate;

// #[ctrl(get_by_id)]
struct OssBucketCtrl;

/// # 根据ID获取记录的信息
///
/// 该接口通过查询参数中的ID获取对应记录的详细信息
///
/// ## 查询参数
/// * `id` - 记录的唯一标识符，类型为u64
///
/// ## 返回值
/// * 成功时返回对应的记录信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少参数`id`时，返回`ValidationError`错误
/// * 当参数`id`格式不正确时，返回`ValidationError`错误
/// * 当根据ID找不到对应记录时，返回相应的错误信息
#[utoipa::path(
    get,
    path = "/oss/bucket/get-by-id/{id}",
    params(
        ("id" = u64, Path, description = "记录的唯一标识符")
    ),
    responses(
        (status = OK, body = Ro<OssBucketVo>)
    )
)]
#[log_call]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<Json<Ro<Model>>, CtrlError> {
    let ro = OssBucketSvc::get_by_id::<DatabaseConnection>(id, None).await?;
    Ok(Json(ro))
}

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
    delete,
    path = "/oss/bucket/cascade/{id}",
    params(
        ("id" = u64, Path, description = "记录的唯一标识符")
    ),
    responses((status = OK, body = Ro<String>))
)]
#[log_call]
pub async fn del_cascade(
    Path(id): Path<u64>,
    headers: HeaderMap,
) -> Result<Json<Ro<Model>>, CtrlError> {
    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(&headers)?;

    let ro = OssBucketSvc::del_cascade::<DatabaseTransaction>(id, current_user_id, None).await?;
    Ok(Json(ro))
}
