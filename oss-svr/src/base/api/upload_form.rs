use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;

/// # 上传表单结构体，用于处理文件上传请求
///
/// 该结构体使用 `actix-multipart` 库来解析 multipart/form-data 请求，
/// 包含一个临时文件和可选的哈希值字段。
///
/// ## 字段
///
/// * `file` - 上传的临时文件
/// * `hash` - 可选的文件哈希值
///
/// ## 示例
///
/// ```rust
/// use actix_multipart::form::MultipartForm;
///
/// // 在实际使用中，该结构体会通过 multipart 表单自动解析
/// MultipartForm(form): MultipartForm<UploadForm>
/// ```
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart]
    pub file: TempFile,
    pub hash: Option<Text<String>>,
}
