use crate::app::oss_config::OssConfig;
use idworker::IdWorkerConfig;
use robotech::db::DbConfig;
use robotech::web::WebServerConfig;
use serde::{Deserialize, Serialize};

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    /// oss
    #[serde(default = "OssConfig::default")]
    pub oss: OssConfig,
    /// db
    #[serde(default = "DbConfig::default")]
    pub db: DbConfig,
    /// Web服务器
    #[serde(default = "WebServerConfig::default")]
    pub web_server: WebServerConfig,
    /// id_worker
    #[serde(default = "IdWorkerConfig::default")]
    pub id_worker: IdWorkerConfig,
}
