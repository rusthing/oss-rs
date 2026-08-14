use anyhow::anyhow;
use reqwest::header::{HeaderMap, HeaderValue};
use robotech::api_client::{ApiClient, ApiClientError};
use robotech::cst::user_id_cst::USER_ID_HEADER_NAME;
use robotech::ro::Ro;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::string::ToString;

/// OSS FILE API
#[derive(Debug)]
pub struct OssFileApiClient {
    pub api_client: ApiClient,
}

impl Deref for OssFileApiClient {
    type Target = ApiClient;

    fn deref(&self) -> &Self::Target {
        &self.api_client
    }
}
impl DerefMut for OssFileApiClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api_client
    }
}

impl OssFileApiClient {
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
    ) -> Result<Ro<serde_json::Value>, ApiClientError> {
        let url = format!("/oss/file/upload/{}", bucket);
        let form = reqwest::multipart::Form::new()
            .file("file", file_path)
            .await
            .map_err(|e| ApiClientError::ReadFile(url.clone(), e))?
            .text("fileName", file_name.to_string());
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_ID_HEADER_NAME,
            HeaderValue::from_str(&current_user_id.to_string().as_str())
                .map_err(|e| anyhow!("current_user_id: {}", e))?,
        );

        self.multipart(&url, form, Some(headers), None).await
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
    ) -> Result<Ro<serde_json::Value>, ApiClientError> {
        let url = format!("/oss/file/upload/{}", bucket);
        let part = reqwest::multipart::Part::bytes(data).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_ID_HEADER_NAME,
            HeaderValue::from_str(&current_user_id.to_string().as_str())
                .map_err(|e| anyhow!("current_user_id: {}", e))?,
        );
        self.multipart(&url, form, Some(headers), None).await
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
    ) -> Result<Vec<u8>, ApiClientError> {
        let url = format!("/oss/file/download/{}", obj_id);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_ID_HEADER_NAME,
            HeaderValue::from_str(&current_user_id.to_string().as_str())
                .map_err(|e| anyhow!("current_user_id: {}", e))?,
        );
        self.get_bytes::<()>(&url, None, Some(headers), None).await
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
    ) -> Result<Vec<u8>, ApiClientError> {
        let url = format!("/oss/file/preview/{}", obj_id);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_ID_HEADER_NAME,
            HeaderValue::from_str(&current_user_id.to_string().as_str())
                .map_err(|e| anyhow!("current_user_id: {}", e))?,
        );
        self.get_bytes::<()>(&url, None, Some(headers), None).await
    }
}
