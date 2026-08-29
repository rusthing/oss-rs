use arc_swap::ArcSwapOption;
use bytesize::ByteSize;
use config::Value;
use robotech::cfg::CfgError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use wheel_rs::config_utils::has_config_changed;

const OSS_CONFIG_KEY: &str = "oss";
static OSS_CONFIG: ArcSwapOption<OssConfig> = ArcSwapOption::const_empty();

pub fn get_oss_config() -> Result<Arc<OssConfig>, CfgError> {
    OSS_CONFIG
        .load_full()
        .ok_or(CfgError::NotInit("OSS config not initialized".to_string()))
}

pub fn setup_oss_config(oss_config: OssConfig, changed: &Option<HashMap<String, Value>>) {
    info!("setup oss config...: {oss_config:?}");
    if changed
        .as_ref()
        .map(|changed| has_config_changed(OSS_CONFIG_KEY, changed))
        .unwrap_or(true)
    {
        OSS_CONFIG.store(Some(Arc::new(oss_config)));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct OssConfig {
    /// 文件根目录
    #[serde(default = "file_root_dir_default")]
    pub file_root_dir: String,
    /// 存储文件目录格式
    #[serde(default = "file_dir_format_default")]
    pub file_dir_format: String,
    /// 上传文件限制的大小
    #[serde(default = "upload_file_limit_size_default")]
    pub upload_file_limit_size: ByteSize,
    /// 上传缓冲区大小
    #[serde(default = "upload_buffer_size_default")]
    pub upload_buffer_size: ByteSize,
    /// 下载缓冲区大小
    #[serde(default = "download_buffer_size_default")]
    pub download_buffer_size: ByteSize,
}

impl Default for OssConfig {
    fn default() -> Self {
        OssConfig {
            file_root_dir: file_root_dir_default(),
            file_dir_format: file_dir_format_default(),
            upload_file_limit_size: upload_file_limit_size_default(),
            upload_buffer_size: upload_buffer_size_default(),
            download_buffer_size: download_buffer_size_default(),
        }
    }
}

fn file_root_dir_default() -> String {
    "storage".to_string()
}

fn file_dir_format_default() -> String {
    "%Y/%m/%d/%H".to_string()
}

fn upload_file_limit_size_default() -> ByteSize {
    ByteSize::mib(300)
}
fn upload_buffer_size_default() -> ByteSize {
    ByteSize::mib(1)
}

fn download_buffer_size_default() -> ByteSize {
    ByteSize::mib(1)
}
