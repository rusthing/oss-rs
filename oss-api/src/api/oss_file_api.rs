use robotech::api::api_settings::ApiSettings;
use robotech::api::base_api::BaseApi;
use robotech::ro::Ro;
use std::fmt::Display;
use std::string::ToString;

/// OSS FILE API
#[derive(Debug)]
pub struct OssFileApi {
    pub api_settings: ApiSettings,
}

impl BaseApi for OssFileApi {
    fn get_api_settings(&self) -> &ApiSettings {
        &self.api_settings
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
    ) -> Result<Ro<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("/oss/file/upload/{}", bucket);
        let form = reqwest::multipart::Form::new()
            .file("file", file_path)
            .await?
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
    ) -> Result<Ro<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("/oss/file/preview/{}", obj_id);
        self.get_bytes(&url, current_user_id).await
    }
}
