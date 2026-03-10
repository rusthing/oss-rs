use crate::dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::svc::OssBucketSvc;
use crate::vo::OssBucketVo;
use robotech_macros::ctrl;

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
