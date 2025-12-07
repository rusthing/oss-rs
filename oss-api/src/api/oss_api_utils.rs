use crate::api::oss_file_api::OssFileApi;
use log::info;
use robotech::api::{ApiSettings, CrudApi};
use std::collections::HashMap;
use std::sync::OnceLock;

pub static OSS_FILE_API: OnceLock<OssFileApi> = OnceLock::new();

/// 初始化OssFileApi
pub fn init_oss_api(api_settings: HashMap<String, ApiSettings>) {
    info!("初始化oss的api");
    let default_settings = ApiSettings {
        base_url: "http://127.0.0.1:9840".to_string(),
    };
    let api_settings = api_settings.get("oss").unwrap_or(&default_settings).clone();
    OSS_FILE_API
        .set(OssFileApi {
            api: CrudApi { api_settings },
        })
        .expect("无法设置OssFileApi的配置信息");
}
