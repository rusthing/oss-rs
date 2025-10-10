use crate::api::api_error::ApiError;
use crate::svc::oss_obj_ref_svc;
use crate::utils::file_utils::calc_hash;
use crate::utils::upload::UploadForm;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use once_cell::sync::Lazy;
use regex::Regex;

/// 上传文件
#[post("/upload/{bucket}")]
pub async fn upload(
    bucket: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, ApiError> {
    let bucket = bucket.into_inner();
    if bucket.is_empty() {
        return Err(ApiError::ValidationError("无效的bucket".to_string()));
    }

    let file_name = form.file.file_name.unwrap();
    let file_size = form.file.size;
    let temp_file = form.file.file;
    let provided_hash: Option<String> = form.hash.map(|t| t.into_inner());
    let computed_hash = calc_hash(&temp_file.path());
    if provided_hash.is_some() && provided_hash.unwrap() != computed_hash {
        return Err(ApiError::ValidationError("文件Hash值不匹配".to_string()));
    }
    let hash = computed_hash;

    let ro = oss_obj_ref_svc::upload(&bucket, &file_name, file_size, &hash, temp_file).await?;

    Ok(HttpResponse::Ok().json(ro))
}

/// 下载文件
#[get("/download/{obj_id}")]
pub async fn download(obj_id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let (obj_id, ext) = parse_obj_id(&obj_id.into_inner())?;

    let (file_name, _file_size, length, content, ..) =
        oss_obj_ref_svc::download(obj_id.parse::<u64>().unwrap(), ext.unwrap(), None, None).await?;

    Ok(response_octet_stream(file_name, length, content))
}

/// 预览文件
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
                .map_err(|_| ApiError::ValidationError("无效的Range".to_string()))?;
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
        .map_err(|_| ApiError::ValidationError("无效的ID".to_string()))?;

    let (file_name, file_size, length, content, start, end) =
        oss_obj_ref_svc::download(obj_id_num, ext.clone().unwrap_or_default(), start, end).await?;

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

fn response_inline(length: u64, content_type: &str, content: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(content_type)
        .append_header(("Content-Disposition", "inline"))
        .append_header(("Content-Length", length.to_string()))
        .body(content)
}

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

static OBJ_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)\.?([a-zA-Z0-9]*)$").unwrap());

fn parse_obj_id(obj_id: &String) -> Result<(String, Option<String>), ApiError> {
    Ok(if let Some(captures) = OBJ_ID_REGEX.captures(&obj_id) {
        // 匹配成功，提取19位数字和后缀
        let obj_id = captures.get(1).unwrap().as_str().to_string();
        let ext = captures.get(2).unwrap().as_str().to_string();
        (obj_id, Some(ext))
    } else {
        return Err(ApiError::ValidationError(format!(
            "路径<id>格式不正确: {}",
            obj_id
        )));
    })
}
