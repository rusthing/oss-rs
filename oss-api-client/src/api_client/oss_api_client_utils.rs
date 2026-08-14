use crate::api_client::oss_file_api_client::OssFileApiClient;
use arc_swap::ArcSwap;
use robotech::api_client::{ApiClient, ApiClientConfig, ApiClientError};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tracing::info;

static OSS_FILE_API_CLIENT: OnceLock<ArcSwap<OssFileApiClient>> = OnceLock::new();

/// 初始化OssFileApi
pub fn init_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
) -> Result<(), ApiClientError> {
    info!("初始化oss的api客户端");
    let default_config = ApiClientConfig {
        base_url: "http://127.0.0.1:9840".to_string(),
    };
    let api_client_config = api_config.get("oss").unwrap_or(&default_config).clone();
    let oss_file_api_client = OssFileApiClient {
        api_client: ApiClient { api_client_config },
    };
    OSS_FILE_API_CLIENT
        .set(ArcSwap::new(Arc::new(oss_file_api_client)))
        .map_err(|_| ApiClientError::SetApiClient("OSS_FILE_API_CLIENT".to_string()))
}

pub fn get_oss_file_api_client() -> Result<Arc<OssFileApiClient>, ApiClientError> {
    Ok(OSS_FILE_API_CLIENT
        .get()
        .ok_or(ApiClientError::GetApiClient(
            "OSS_FILE_API_CLIENT".to_string(),
        ))?
        .load_full()
        .clone())
}

pub fn update_oss_file_api_client(
    oss_file_api_client: OssFileApiClient,
) -> Result<(), ApiClientError> {
    if let Some(swap) = OSS_FILE_API_CLIENT.get() {
        swap.store(Arc::new(oss_file_api_client));
        Ok(())
    } else {
        Err(ApiClientError::NotInit("OSS_FILE_API_CLIENT".to_string()))
    }
}
