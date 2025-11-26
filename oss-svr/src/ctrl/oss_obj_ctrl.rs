use crate::svc::oss_obj_svc::OssObjSvc;
use crate::dto::oss_obj_dto::{OssObjAddDto, OssObjModifyDto, OssObjSaveDto};
use crate::vo::oss_obj::OssObjVo;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Result};
use robotech::ctrl::ctrl_error::CtrlError;
use robotech::ctrl::ctrl_utils::{get_current_user_id, get_id_from_query_params};
use robotech::ro::Ro;
use std::collections::HashMap;
use validator::Validate;

/// # 添加新的记录
///
/// 该接口用于添加一个新的记录
///
/// ## 请求体
/// * `OssObjAddDto` - 包含记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回添加后的信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[utoipa::path(
    path = "/oss/obj",
    responses((status = OK, body = Ro<OssObjVo>))
)]
#[post("")]
pub async fn add(
    json_body: web::Json<OssObjAddDto>,
    req: HttpRequest,
) -> Result<HttpResponse, CtrlError> {
    let mut to = json_body.into_inner();

    to.validate()?;

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    to.current_user_id = get_current_user_id(req)?;

    let result = OssObjSvc::add(to, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 修改记录的信息
///
/// 该接口用于修改一个已存在记录的信息
///
/// ## 请求体
/// * `OssObjModifyDto` - 包含待修改记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回修改后的信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[utoipa::path(
    path = "/oss/obj",
    responses((status = OK, body = Ro<OssObjVo>))
)]
#[put("")]
pub async fn modify(
    json_body: web::Json<OssObjModifyDto>,
    req: HttpRequest,
) -> Result<HttpResponse, CtrlError> {
    let mut to = json_body.into_inner();

    to.validate()?;

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    to.current_user_id = get_current_user_id(req)?;

    let result = OssObjSvc::modify(to, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 保存记录的信息
///
/// 该接口用于保存记录的信息，如果记录不存在则创建新记录，如果记录已存在则更新记录
///
/// ## 请求体
/// * `OssObjSaveDto` - 包含记录信息的结构体
///
/// ## 请求头
/// * `USER_ID_HEADER_NAME` - 当前用户ID，必需项，类型为u64
///
/// ## 返回值
/// * 成功时返回保存后的信息的JSON格式数据
/// * 失败时返回相应的错误信息
///
/// ## 错误处理
/// * 当缺少必要参数时，返回`ValidationError`错误
/// * 当参数格式不正确时，返回`ValidationError`错误
/// * 其他业务逻辑错误将按相应规则处理
#[utoipa::path(
    path = "/oss/obj/save",
    responses((status = OK, body = Ro<OssObjVo>))
)]
#[post("/save")]
pub async fn save(
    json_body: web::Json<OssObjSaveDto>,
    req: HttpRequest,
) -> Result<HttpResponse, CtrlError> {
    let mut to = json_body.into_inner();

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    to.current_user_id = get_current_user_id(req)?;

    let result = OssObjSvc::save(to, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// # 删除记录
///
/// 该接口用于删除一个已存在的记录
///
/// ## 请求参数
/// * `id` - 待删除记录的唯一标识符，类型为u64
///
/// ## 错误处理
/// * 当缺少参数`id`时，返回`ValidationError`错误
/// * 当参数`id`格式不正确时，返回`ValidationError`错误
/// * 当根据ID找不到对应记录时，返回相应的错误信息
#[utoipa::path(
    path = "/oss/obj",
    params(
        ("id", description = "记录的唯一标识符，类型为u64")
    ),
    responses((status = OK, body = Ro<String>))
)]
#[delete("")]
pub async fn del(
    query: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> Result<HttpResponse, CtrlError> {
    let id = get_id_from_query_params(query)?;

    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(req)?;
    Ok(HttpResponse::Ok().json(OssObjSvc::del(id, current_user_id, None).await?))
}

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
    path = "/oss/obj/get-by-id",
    params(
        ("id", description = "记录的唯一标识符，类型为u64")
    ),
    responses(
        (status = OK, body = Ro<OssObjVo>)
    )
)]
#[get("/get-by-id")]
pub async fn get_by_id(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, CtrlError> {
    let id = get_id_from_query_params(query)?;

    let ro = OssObjSvc::get_by_id(id, None).await?;
    Ok(HttpResponse::Ok().json(ro))
}
