use crate::api_client::oss_file_api_client::OssFileApiClient;
use arc_swap::ArcSwapOption;
use config::Value;
use robotech::api_client::{ApiClient, ApiClientConfig, API_CLIENT_CONFIG_KEY};
use robotech::cfg::CfgError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use wheel_rs::config_utils::has_config_changed;

static OSS_API_CLIENT: ArcSwapOption<OssApiClient> = ArcSwapOption::const_empty();

pub struct OssApiClient {
    pub file_client: OssFileApiClient,
}

pub fn get_oss_api_client() -> Result<Arc<OssApiClient>, CfgError> {
    OSS_API_CLIENT.load_full().ok_or(CfgError::NotInit(
        "OSS_API_CLIENT not initialized".to_string(),
    ))
}

pub fn setup_oss_api_client(
    api_config: HashMap<String, ApiClientConfig>,
    changed: &Option<HashMap<String, Value>>,
) {
    info!("setup oss api client...: {api_config:?}");
    if changed
        .as_ref()
        .map(|changed| has_config_changed(API_CLIENT_CONFIG_KEY, changed))
        .unwrap_or(true)
    {
        let oss_api_client = new_oss_api_client_from_config(api_config);
        OSS_API_CLIENT.store(Some(Arc::new(oss_api_client)));
    }
}

fn new_oss_api_client_from_config(api_config: HashMap<String, ApiClientConfig>) -> OssApiClient {
    let default_config = ApiClientConfig {
        base_url: "http://127.0.0.1:9840".to_string(),
    };
    let api_client_config = api_config
        .get(API_CLIENT_CONFIG_KEY)
        .unwrap_or(&default_config)
        .clone();
    OssApiClient {
        file_client: OssFileApiClient {
            api_client: ApiClient { api_client_config },
        },
    }
}
