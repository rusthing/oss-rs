use crate::api_client::oss_file_api_client::OssFileApiClient;
use arc_swap::ArcSwap;
use robotech::api_client::{ApiClient, ApiClientConfig, ApiClientError};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tracing::info;

static OSS_API_CLIENT: OnceLock<ArcSwap<OssApiClient>> = OnceLock::new();

pub struct OssApiClient {
    pub oss_file_api_client: OssFileApiClient,
}

/// 初始化OssFileApi
pub fn init_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
) -> Result<(), ApiClientError> {
    info!("初始化oss的api客户端");
    let oss_api_client = new_oss_api_client_from_config(api_config);
    OSS_API_CLIENT
        .set(ArcSwap::new(Arc::new(oss_api_client)))
        .map_err(|_| ApiClientError::SetApiClient("OSS_API_CLIENT".to_string()))
}

pub fn get_oss_api_client() -> Result<Arc<OssApiClient>, ApiClientError> {
    Ok(OSS_API_CLIENT
        .get()
        .ok_or(ApiClientError::GetApiClient("OSS_API_CLIENT".to_string()))?
        .load_full()
        .clone())
}

pub fn update_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
) -> Result<(), ApiClientError> {
    info!("更新oss的api客户端");
    let oss_api_client = new_oss_api_client_from_config(api_config);
    if let Some(swap) = OSS_API_CLIENT.get() {
        swap.store(Arc::new(oss_api_client));
        Ok(())
    } else {
        Err(ApiClientError::NotInit("OSS_API_CLIENT".to_string()))
    }
}

fn new_oss_api_client_from_config(api_config: HashMap<String, ApiClientConfig>) -> OssApiClient {
    let default_config = ApiClientConfig {
        base_url: "http://127.0.0.1:9840".to_string(),
    };
    let api_client_config = api_config.get("oss").unwrap_or(&default_config).clone();
    OssApiClient {
        oss_file_api_client: OssFileApiClient {
            api_client: ApiClient { api_client_config },
        },
    }
}
