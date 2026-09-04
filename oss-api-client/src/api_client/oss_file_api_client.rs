use anyhow::anyhow;
use reqwest::header::{HeaderMap, HeaderValue};
use robotech::api_client::ApiClientError;
use robotech::cst::user_id_cst::USER_ID_HEADER_NAME;
use robotech::micro_svc::FeignClient;
use robotech::ro::Ro;
use std::fmt::Display;

pub struct OssFileApiClient {
    pub feign_client: FeignClient,
}

impl OssFileApiClient {
    pub fn new(feign_client: FeignClient) -> Self {
        Self { feign_client }
    }

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

        self.feign_client
            .multipart::<String>(&url, form, Some(headers))
            .await
    }

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
        self.feign_client
            .multipart::<String>(&url, form, Some(headers))
            .await
    }

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
        self.feign_client
            .get_bytes::<()>(&url, None, Some(headers))
            .await
    }

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
        self.feign_client
            .get_bytes::<()>(&url, None, Some(headers))
            .await
    }
}