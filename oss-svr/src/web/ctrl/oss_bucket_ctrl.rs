use crate::dto::oss_bucket_dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::svc::oss_bucket_svc::OssBucketSvc;
use crate::vo::oss_bucket_vo::OssBucketVo;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::Json;
use robotech::macros::log_call;
use robotech::ro::Ro;
use robotech::web::ctrl_utils::get_current_user_id;
use robotech::web::CtrlError;
use robotech_macros::ctrl;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
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
    delete,
    path = "/oss/bucket/cascade/{id}",
    params(
        ("id" = u64, Path, description = "记录的唯一标识符")
    ),
    responses((status = OK, body = Ro<OssBucketVo>))
)]
#[log_call]
pub async fn del_cascade(
    Path(id): Path<u64>,
    headers: HeaderMap,
) -> Result<Json<Ro<OssBucketVo>>, CtrlError> {
    let ro = OssBucketSvc::del_cascade::<DatabaseTransaction>(id, None).await?;
    Ok(Json(ro))
}
