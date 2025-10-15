use crate::base::api::api_error::ApiError;
use crate::base::api::api_utils::get_current_user_id;
use crate::ro::ro::Ro;
use crate::svc::oss_file_svc::OssFileSvc;
use crate::utils::file_utils::calc_hash;
use crate::base::api::upload_form::UploadForm;
use crate::vo::oss_obj_ref::OssObjRefVo;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use once_cell::sync::Lazy;
use regex::Regex;

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
    path = "/oss/file/upload/{bucket}",
    params(
        ("bucket" = String, Path, description = "存储桶名称")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[post("/upload/{bucket}")]
pub async fn upload(
    bucket: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    // 从header中解析当前用户ID，如果没有或解析失败则抛出ApiError
    let current_user_id = get_current_user_id(req)?;

    let bucket = bucket.into_inner();
    if bucket.is_empty() {
        return Err(ApiError::from(validator::ValidationError::new(
            "缺少必要路径<桶名称>",
        )));
    }

    let file_name = form.file.file_name.unwrap();
    let file_size = form.file.size;
    let temp_file = form.file.file;
    let provided_hash: Option<String> = form.hash.map(|t| t.into_inner());
    let computed_hash = calc_hash(&temp_file.path());
    if provided_hash.is_some() && provided_hash.unwrap() != computed_hash {
        return Err(ApiError::from(validator::ValidationError::new(
            "文件Hash值不匹配",
        )));
    }
    let hash = computed_hash;

    let ro = OssFileSvc::upload(
        &bucket,
        &file_name,
        file_size,
        &hash,
        temp_file,
        current_user_id,
        None,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ro))
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
    path = "/oss/file/download/{obj_id}",
    params(
        ("obj_id" = String, Path, description = "对象ID")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[get("/download/{obj_id}")]
pub async fn download(obj_id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let (obj_id, ext) = parse_obj_id(&obj_id.into_inner())?;

    let (file_name, _file_size, length, content, ..) = OssFileSvc::download(
        obj_id.parse::<u64>().unwrap(),
        ext.unwrap(),
        None,
        None,
        None,
    )
    .await?;

    Ok(response_octet_stream(file_name, length, content))
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
    path = "/oss/file/preview/{obj_id}",
    params(
        ("obj_id" = String, Path, description = "对象ID")
    ),
    responses((status = OK, body = Ro<OssObjRefVo>))
)]
#[get("/preview/{obj_id}")]
pub async fn preview(
    req: HttpRequest,
    obj_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let (obj_id, ext) = parse_obj_id(&obj_id.into_inner())?;

    // 是否有Range
    let (start, end) = match req.headers().get("Range") {
        Some(range) => {
            let range = range
                .to_str()
                .map_err(|_| ApiError::from(validator::ValidationError::new("无效的Range")))?;
            let parts: Vec<&str> = range.split("=").nth(1).unwrap().split("-").collect();
            let start = parts[0].parse::<u64>().ok();
            let end = if parts.len() > 1 && !parts[1].is_empty() {
                parts[1].parse::<u64>().ok()
            } else {
                None
            };
            (start, end)
        }
        None => (None, None),
    };

    let obj_id_num = obj_id
        .parse::<u64>()
        .map_err(|_| ApiError::from(validator::ValidationError::new("无效的ID")))?;

    let (file_name, file_size, length, content, start, end) = OssFileSvc::download(
        obj_id_num,
        ext.clone().unwrap_or_default(),
        start,
        end,
        None,
    )
    .await?;

    match ext.as_deref() {
        Some(ext) => {
            let content_type = match ext {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                "pdf" => "application/pdf",
                "txt" | "md" => "text/plain",
                "mp3" | "wav" | "ogg" | "aac" | "flac" => "audio/mpeg",
                "mp4" => "video/mp4",
                _ => return Ok(response_octet_stream(file_name, length, content)),
            };

            Ok(if let (Some(start_pos), Some(end_pos)) = (start, end) {
                response_partial_content(file_size, content_type, start_pos, end_pos, content)
            } else {
                response_inline(length, content_type, content)
            })
        }
        None => Ok(response_octet_stream(file_name, length, content)),
    }
}

/// # 创建部分内容响应
///
/// 该函数用于创建HTTP 206 Partial Content响应，主要用于支持Range请求的文件预览功能
///
/// ## 参数
/// - `file_size`: 完整文件的总大小（字节）
/// - `content_type`: 响应内容的MIME类型
/// - `start`: 请求范围的起始位置（字节偏移量）
/// - `end`: 请求范围的结束位置（字节偏移量）
/// - `content`: 实际返回的内容数据
///
/// ## 返回值
/// 返回配置了适当HTTP头部的PartialContent响应，包括：
/// - Content-Type: 内容类型
/// - Content-Disposition: 设置为inline以便浏览器内联显示
/// - Content-Length: 文件总长度
/// - Accept-Ranges: 表明服务器支持byte范围请求
/// - Content-Range: 指定当前返回内容的字节范围
fn response_partial_content(
    file_size: u64,
    content_type: &str,
    start: u64,
    end: u64,
    content: Vec<u8>,
) -> HttpResponse {
    HttpResponse::PartialContent()
        .content_type(content_type)
        .append_header(("Content-Disposition", "inline"))
        .append_header(("Content-Length", file_size.to_string()))
        .append_header(("Accept-Ranges", "bytes"))
        .append_header((
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, file_size),
        ))
        .body(content)
}

/// # 创建内联内容响应
///
/// 该函数用于创建HTTP 200 OK响应，将内容以内联方式返回，适用于浏览器可以直接预览的文件类型
///
/// ## 参数
/// - `length`: 返回内容的实际长度（字节）
/// - `content_type`: 响应内容的MIME类型
/// - `content`: 实际返回的内容数据
///
/// ## 返回值
/// 返回配置了适当HTTP头部的OK响应，包括：
/// - Content-Type: 内容类型
/// - Content-Disposition: 设置为inline以便浏览器内联显示
/// - Content-Length: 实际内容长度
fn response_inline(length: u64, content_type: &str, content: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(content_type)
        .append_header(("Content-Disposition", "inline"))
        .append_header(("Content-Length", length.to_string()))
        .body(content)
}

/// # 创建二进制流响应
///
/// 该函数用于创建HTTP 200 OK响应，将内容作为二进制流返回，适用于浏览器应该下载而非直接显示的文件类型
///
/// ## 参数
/// - `file_name`: 文件名，用于设置Content-Disposition头部，提示浏览器保存时使用的文件名
/// - `length`: 返回内容的实际长度（字节）
/// - `content`: 实际返回的内容数据
///
/// ## 返回值
/// 返回配置了适当HTTP头部的OK响应，包括：
/// - Content-Type: 设置为application/octet-stream表示二进制流
/// - Content-Disposition: 设置为attachment并附带文件名，提示浏览器下载文件
/// - Content-Length: 实际内容长度
fn response_octet_stream(file_name: String, length: u64, content: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_name),
        ))
        .append_header(("Content-Length", length.to_string()))
        .body(content)
}

/// # 正则表达式，用于匹配对象ID的格式(19位数字+可选的扩展名)
static OBJ_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)\.?([a-zA-Z0-9]*)$").unwrap());

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
fn parse_obj_id(obj_id: &String) -> Result<(String, Option<String>), ApiError> {
    Ok(if let Some(captures) = OBJ_ID_REGEX.captures(&obj_id) {
        // 匹配成功，提取19位数字和后缀
        let obj_id = captures.get(1).unwrap().as_str().to_string();
        let ext = captures.get(2).unwrap().as_str().to_string();
        (obj_id, Some(ext))
    } else {
        let msg = format!("路径<id>格式不正确: {}", obj_id);
        return Err(ApiError::from(validator::ValidationError::new(Box::leak(
            msg.into_boxed_str(),
        ))));
    })
}
