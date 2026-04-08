use crate::app::oss_config::OssConfig;
use idworker::IdWorkerConfig;
use robotech::app::AppError;
use robotech::db::DbConnConfig;
use robotech::web::WebServerConfig;
use serde::Deserialize;
use std::sync::RwLock;

static APP_CONFIG: RwLock<Option<AppConfig>> = RwLock::new(None);

/// 获取App配置的只读访问
pub fn get_app_config() -> Result<AppConfig, AppError> {
    let read_lock = APP_CONFIG.read().map_err(|_| AppError::GetAppConfig())?;
    read_lock.clone().ok_or(AppError::GetAppConfig())
}

/// 设置App配置
pub fn set_app_config(value: AppConfig) -> Result<(), AppError> {
    let mut write_lock = APP_CONFIG.write().map_err(|_| AppError::SetAppConfig())?;
    *write_lock = Some(value);
    Ok(())
}

/// 配置文件结构
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    /// oss
    #[serde(default = "OssConfig::default")]
    pub oss: OssConfig,
    /// db
    #[serde(default = "DbConnConfig::default")]
    pub db: DbConnConfig,
    /// Web服务器
    #[serde(default = "WebServerConfig::default")]
    pub web_server: WebServerConfig,
    /// id_worker
    #[serde(default = "IdWorkerConfig::default")]
    pub id_worker: IdWorkerConfig,
}
