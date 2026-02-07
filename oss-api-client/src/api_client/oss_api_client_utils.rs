use crate::api_client::oss_file_api_client::OssFileApiClient;
use log::info;
use robotech::api_client::{ApiClientConfig, ApiClientError, CrudApiClient};
use std::collections::HashMap;
use std::sync::OnceLock;

pub static OSS_FILE_API_CLIENT: OnceLock<OssFileApiClient> = OnceLock::new();

/// 初始化OssFileApi
pub fn init_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
) -> Result<(), ApiClientError> {
    info!("初始化oss的api客户端");
    let default_config = ApiClientConfig {
        base_url: "http://127.0.0.1:9840".to_string(),
    };
    let api_client_config = api_config.get("oss").unwrap_or(&default_config).clone();
    OSS_FILE_API_CLIENT
        .set(OssFileApiClient {
            api_client: CrudApiClient { api_client_config },
        })
        .map_err(|_| ApiClientError::SetApiClient("OSS_FILE_API_CLIENT".to_string()))?;
    Ok(())
}
