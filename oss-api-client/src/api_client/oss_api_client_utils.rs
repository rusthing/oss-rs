use crate::api_client::oss_file_api_client::OssFileApiClient;
use arc_swap::ArcSwapOption;
use config::Value;
use robotech::api_client::{ApiClientConfig, API_CLIENT_CONFIG_KEY};
use robotech::cfg::CfgError;
use robotech::micro_svc::FeignClient;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use wheel_rs::config_utils::has_config_changed;

static OSS_API_CLIENT: ArcSwapOption<OssApiClient> = ArcSwapOption::const_empty();

pub struct OssApiClient {
    pub file_client: OssFileApiClient,
}

pub fn get_oss_api_client() -> Result<Arc<OssApiClient>, CfgError> {
    OSS_API_CLIENT
        .load_full()
        .ok_or(CfgError::NotInit("OSS_API_CLIENT not initialized".to_string()))
}

pub async fn setup_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
    changed: &Option<HashMap<String, Value>>,
) {
    info!("setup oss api client...: {api_config:?}");
    if changed
        .as_ref()
        .map(|changed| has_config_changed(API_CLIENT_CONFIG_KEY, changed))
        .unwrap_or(true)
    {
        let oss_api_client = FeignClient::new_default("oss-svr").await;
        OSS_API_CLIENT.store(Some(Arc::new(OssApiClient {
            file_client: OssFileApiClient::new(oss_api_client),
        })));
    }
}