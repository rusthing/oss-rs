use crate::svc::OssFileSvc;
use crate::vo::OssObjRefVo;
use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use regex::Regex;
use robotech::macros::log_call;
use robotech::ro::Ro;
use robotech::web::CtrlError;
use robotech::web::ctrl_utils::get_current_user_id;
use sea_orm::DatabaseTransaction;
use std::sync::LazyLock;

/// # 上传文件到指定的存储桶
///
/// 该接口接收一个文件和可选的哈希值，将其上传到指定的存储桶中。
/// 如果提供了哈希值，会与计算出的文件哈希值进行比对，确保文件完整性。
///
/// ## 参数
/// - `bucket`: 路径参数，指定文件上传的目标存储桶名称
/// - `form`: Multipart表单数据，包含上传的文件和其他元数据
///
/// ## 返回值
/// 成功时返回包含文件引用信息的`Ro<OssObjRefVo>`对象
///
/// ## 错误处理
/// - 如果存储桶名称为空，返回验证错误
/// - 如果提供的哈希值与计算出的哈希值不匹配，返回验证错误
#[utoipa::path(
    post,
    path = "/oss/file/upload/{bucket}",
    params(
        ("bucket" = String, Path, description = "存储桶名称")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[debug_handler]
#[log_call]
pub async fn upload(
    Path(bucket): Path<String>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Result<Json<Ro<OssObjRefVo>>, CtrlError> {
    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(&headers)?;

    Ok(Json(
        OssFileSvc::upload::<DatabaseTransaction>(&bucket, multipart, current_user_id, None)
            .await?,
    ))
}

/// # 下载模式
#[derive(PartialEq)]
enum DownloadMode {
    /// 下载
    Download,
    /// 预览
    Preview,
}

/// # 下载文件
///
/// 该接口根据对象ID下载对应的文件内容。
///
/// ## 参数
/// - `obj_id`: 路径参数，指定要下载的对象ID，格式为数字ID加可选的文件扩展名后缀(如: 12345.jpg)
///
/// ## 返回值
/// 成功时返回文件的二进制内容，以及适当的HTTP头部信息
///
/// ## 错误处理
/// - 如果对象ID格式不正确，返回验证错误
/// - 如果找不到对应的对象，由服务层返回相应错误
#[utoipa::path(
    get,
    path = "/oss/file/download/{obj_id}",
    params(
        ("obj_id" = String, Path, description = "对象ID")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[debug_handler]
#[log_call]
pub async fn download(
    Path(obj_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, CtrlError> {
    download_or_preview(DownloadMode::Download, obj_id, headers).await
}

/// # 预览文件
///
/// 该接口根据对象ID预览对应的文件内容，支持Range请求用于视频、音频等大文件的分段加载
///
/// ## 参数
/// - `obj_id`: 路径参数，指定要预览的对象ID，格式为数字ID加可选的文件扩展名后缀(如: 12345.jpg)
/// - `req`: HTTP请求对象，用于检查是否包含Range头以支持部分内容请求
///
/// ## 返回值
/// 成功时根据文件类型返回适当的HTTP响应：
/// - 对于图片、PDF、文本等类型文件，以内联方式返回便于浏览器直接预览
/// - 对于其他类型文件，作为附件下载
/// - 支持HTTP Range请求，实现断点续传和流媒体播放
///
/// ## 错误处理
/// - 如果对象ID格式不正确，返回验证错误
/// - 如果Range头格式不正确，返回验证错误
/// - 如果ID不是有效数字，返回验证错误
/// - 如果找不到对应的对象，由服务层返回相应错误
#[utoipa::path(
    get,
    path = "/oss/file/preview/{obj_id}",
    params(
        ("obj_id" = String, Path, description = "对象ID")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[debug_handler]
#[log_call]
pub async fn preview(
    Path(obj_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, CtrlError> {
    download_or_preview(DownloadMode::Preview, obj_id, headers).await
}

async fn download_or_preview(
    mode: DownloadMode,
    obj_id: String,
    headers: HeaderMap,
) -> Result<Response, CtrlError> {
    let (obj_id, ext) = parse_obj_id(&obj_id)?;
    let (file_name, ext, file_size, chunk_size, body, start, end) =
        OssFileSvc::download::<DatabaseTransaction>(headers, obj_id, ext, None).await?;

    let content_type = if mode == DownloadMode::Download {
        "application/octet-stream"
    } else {
        if let Some(content_type) = OssFileSvc::get_content_type_of_preview(&ext) {
            content_type
        } else {
            "application/octet-stream"
        }
    };
    let content_disposition = if content_type == "application/octet-stream" {
        format!("attachment; filename=\"{}\"", file_name).to_string()
    } else {
        "inline".to_string()
    };

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes")); // 默认告知客户端本服务器支持 Range 请求
    response_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    response_headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(content_disposition.as_str())?,
    );

    let (status_code, content_length) = if let (Some(start_pos), Some(end_pos)) = (start, end) {
        response_headers.insert(
            header::CONTENT_RANGE,
            HeaderValue::from_str(
                format!("bytes {}-{}/{}", start_pos, end_pos, file_size).as_str(),
            )?,
        );
        (StatusCode::PARTIAL_CONTENT, file_size)
    } else {
        (StatusCode::OK, chunk_size)
    };
    response_headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(content_length.to_string().as_str())?,
    );

    Ok((status_code, response_headers, body).into_response())
}

/// # 正则表达式，用于匹配对象ID的格式(19位数字+可选的扩展名)
static OBJ_ID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\.?([a-zA-Z0-9]*)$").unwrap());

/// # 解析对象ID
///
/// 该函数用于解析对象ID，将19位数字和可选的扩展名解析为对象ID和扩展名
///
/// ## 参数
/// - `obj_id`: 对象ID，格式为19位数字和可选的扩展名，如"123456789012345.jpg"
///
/// ## 返回值
/// 返回一个元组，包含对象ID和扩展名。如果对象ID格式不正确，则返回错误
///
/// ## 错误处理
/// - 如果对象ID格式不正确，则返回`ApiError::ValidationError`错误
fn parse_obj_id(obj_id: &str) -> Result<(u64, Option<String>), CtrlError> {
    Ok(if let Some(captures) = OBJ_ID_REGEX.captures(&obj_id) {
        // 匹配成功，提取19位数字和后缀
        let obj_id = captures.get(1).unwrap().as_str().to_string();
        let obj_id = obj_id
            .parse::<u64>()
            .map_err(|_| validator::ValidationError::new("<id>格式不正确"))?;
        let ext = if let Some(ext) = captures.get(2) {
            Some(ext.as_str().to_string())
        } else {
            None
        };

        (obj_id, ext)
    } else {
        Err(validator::ValidationError::new("<id>格式不正确"))?
    })
}
