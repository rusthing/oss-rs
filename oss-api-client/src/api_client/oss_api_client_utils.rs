use crate::api_client::oss_file_api_client::OssFileApiClient;
use arc_swap::ArcSwapOption;
use config::Value;
use robotech::api_client::ApiClient;
use robotech::api_client::{ApiClientConfig, API_CLIENT_CONFIG_KEY};
use robotech::cfg::CfgError;
use robotech::micro_svc::FeignClient;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use wheel_rs::config_utils::has_config_changed;

static OSS_API_CLIENT: ArcSwapOption<OssApiClient> = ArcSwapOption::const_empty();
const OSS_API_CLIENT_CONFIG_KEY: &str = "oss";

pub struct OssApiClient {
    pub file_client: OssFileApiClient,
}

pub fn get_oss_api_client() -> Result<Arc<OssApiClient>, CfgError> {
    OSS_API_CLIENT.load_full().ok_or(CfgError::NotInit(
        "OSS_API_CLIENT not initialized".to_string(),
    ))
}

pub async fn setup_oss_api_client(
    apis_config: HashMap<String, ApiClientConfig>,
    changed: &Option<HashMap<String, Value>>,
) -> Result<(), CfgError> {
    info!("setup oss api client...: {apis_config:?} {changed:?}");
    if changed
        .as_ref()
        .map(|changed| has_config_changed(API_CLIENT_CONFIG_KEY, changed))
        .unwrap_or(true)
    {
        let mut oss_api_client: Option<OssFileApiClient> = None;
        for (key, api_client_config) in apis_config {
            if key == OSS_API_CLIENT_CONFIG_KEY {
                if let Some(ref feign_svc) = api_client_config.svc_name {
                    info!("feign mode: using service discovery for '{}'", feign_svc);
                    oss_api_client = Some(OssFileApiClient::new_feign(
                        FeignClient::new_default(feign_svc).await,
                    ));
                } else {
                    let base_url = api_client_config
                        .base_url
                        .ok_or(CfgError::NotInit("base_url not initialized".to_string()))?;
                    info!("static mode: using base_url '{base_url}' for '{key}'",);
                    oss_api_client = Some(OssFileApiClient::new_static(ApiClient::new(base_url)));
                }
            }
        }
        let oss_api_client = oss_api_client.ok_or(CfgError::NotInit(
            "OSS API client not initialized".to_string(),
        ))?;

        OSS_API_CLIENT.store(Some(Arc::new(OssApiClient {
            file_client: oss_api_client,
        })));
    }
    Ok(())
}
