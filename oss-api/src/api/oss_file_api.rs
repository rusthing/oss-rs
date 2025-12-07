use robotech::api::ApiError::FileError;
use robotech::api::{ApiError, CrudApi};
use robotech::ro::Ro;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::string::ToString;

/// OSS FILE API
#[derive(Debug)]
pub struct OssFileApi {
    pub api: CrudApi,
}

impl Deref for OssFileApi {
    type Target = CrudApi;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}
impl DerefMut for OssFileApi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api
    }
}

impl OssFileApi {
    /// # 上传文件到指定的存储桶
    ///
    /// ## 参数
    /// * `bucket` - 存储桶名称
    /// * `file_path` - 要上传的本地文件路径
    /// * `file_name` - 上传后的文件名
    ///
    /// ## 返回值
    /// 返回上传结果
    pub async fn upload_file(
        &self,
        bucket: &str,
        file_path: &str,
        file_name: &str,
        current_user_id: u64,
    ) -> Result<Ro<serde_json::Value>, ApiError> {
        let url = format!("/oss/file/upload/{}", bucket);
        let form = reqwest::multipart::Form::new()
            .file("file", file_path)
            .await
            .map_err(|e| FileError(url.clone(), e))?
            .text("fileName", file_name.to_string());

        self.multipart(&url, form, current_user_id).await
    }

    /// # 上传文件内容到指定的存储桶
    ///
    /// ## 参数
    /// * `bucket` - 存储桶名称
    /// * `file_path` - 要上传的本地文件路径
    /// * `file_name` - 上传后的文件名
    /// * `data` - 文件内容
    ///
    /// ## 返回值
    /// 返回上传结果
    pub async fn upload_file_content(
        &self,
        bucket: &str,
        file_name: &str,
        data: Vec<u8>,
        current_user_id: u64,
    ) -> Result<Ro<serde_json::Value>, ApiError> {
        let url = format!("/oss/file/upload/{}", bucket);
        let part = reqwest::multipart::Part::bytes(data).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);
        self.multipart(&url, form, current_user_id).await
    }

    /// 下载文件
    ///
    /// # Arguments
    ///
    /// * `obj_id` - 对象ID
    ///
    /// # Returns
    ///
    /// 返回下载的文件内容
    pub async fn download_file(
        &self,
        obj_id: impl Display,
        current_user_id: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let url = format!("/oss/file/download/{}", obj_id);
        self.get_bytes(&url, current_user_id).await
    }

    /// # 预览文件
    ///
    /// ## Arguments
    ///
    /// * `obj_id` - 对象ID
    ///
    /// ## Returns
    ///
    /// 返回预览的文件内容
    pub async fn preview_file(
        &self,
        obj_id: impl Display,
        current_user_id: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let url = format!("/oss/file/preview/{}", obj_id);
        self.get_bytes(&url, current_user_id).await
    }
}
