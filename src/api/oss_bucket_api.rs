use crate::svc::oss_bucket_svc;
use crate::to::oss_bucket::{OssBucketAddTo, OssBucketModifyTo, OssBucketSaveTo};
use crate::utils::api_utils::{get_current_user_id, ApiError};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;

/// # 添加新的记录
///
/// 该接口用于添加一个新的记录信息。
///
/// ## 请求体
/// * `AddTo` - 包含记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回添加后的记录信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[post("")]
pub async fn add(
    json_body: web::Json<OssBucketAddTo>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let mut bucket = json_body.into_inner();

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    bucket.current_user_id = get_current_user_id(req)?;

    let result = oss_bucket_svc::add(bucket, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 修改记录信息
///
/// 该接口用于修改一个已存在的记录信息。
///
/// ## 请求体
/// * `ModifyTo` - 包含待修改记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回修改后的记录信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[put("")]
pub async fn modify(
    json_body: web::Json<OssBucketModifyTo>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let mut bucket = json_body.into_inner();

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    bucket.current_user_id = get_current_user_id(req)?;

    let result = oss_bucket_svc::modify(bucket, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 保存记录信息
///
/// 该接口用于保存记录信息，如果记录不存在则创建新记录，如果记录已存在则更新记录。
///
/// ## 请求体
/// * `SaveTo` - 包含记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回保存后的记录信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[post("/save")]
pub async fn save(
    json_body: web::Json<OssBucketSaveTo>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let mut bucket = json_body.into_inner();

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    bucket.current_user_id = get_current_user_id(req)?;

    let result = oss_bucket_svc::save(bucket, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 删除记录信息
///
/// 该接口用于删除一个已存在的记录信息。
///
/// ## 请求参数
/// * `id` - 待删除记录的唯一标识符，类型为u64
///
/// ## 错误处理
/// * 当缺少参数`id`时，返回`ValidationError`错误
/// * 当参数`id`格式不正确时，返回`ValidationError`错误
/// * 当根据ID找不到对应记录时，返回相应的错误信息
#[delete("")]
pub async fn del(
    query: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => {
                return Err(ApiError::ValidationError(format!(
                    "参数<id>格式错误: {}",
                    id_str
                )));
            }
        },
        None => {
            return Err(ApiError::ValidationError("缺少必要参数<id>".to_string()));
        }
    };
    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(req)?;
    Ok(HttpResponse::Ok().json(oss_bucket_svc::del(id, current_user_id, None).await?))
}

/// # 根据ID获取记录信息
///
/// 该接口通过查询参数中的ID获取对应的记录的详细信息。
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
#[get("/get-by-id")]
pub async fn get_by_id(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let id = match query.get("id") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id_val) => id_val,
            Err(_) => {
                return Err(ApiError::ValidationError(format!(
                    "参数<id>格式错误: {}",
                    id_str
                )));
            }
        },
        None => {
            return Err(ApiError::ValidationError("缺少必要参数<id>".to_string()));
        }
    };
    let ro = oss_bucket_svc::get_by_id(id, None).await?;
    Ok(HttpResponse::Ok().json(ro))
}
