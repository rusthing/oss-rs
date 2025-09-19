use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart]
    pub file: TempFile,
    pub hash: Option<Text<String>>,
}
