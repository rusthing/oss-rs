use crate::settings::oss_settings::OssSettings;
use idworker::IdWorkerSettings;
use log::info;
use robotech::db::DbSettings;
use robotech::settings::get_settings;
use robotech::web_server::WebServerSettings;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// 全局配置
pub static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    /// oss
    #[serde(default = "OssSettings::default")]
    pub oss: OssSettings,
    /// db
    #[serde(default = "DbSettings::default")]
    pub db: DbSettings,
    /// Web服务器
    #[serde(default = "WebServerSettings::default")]
    pub web_server: WebServerSettings,
    /// id_worker
    #[serde(default = "IdWorkerSettings::default")]
    pub id_worker: IdWorkerSettings,
}

/// # 创建新的配置实例
///
/// 该函数用于初始化应用程序配置，支持通过配置文件路径和端口参数来定制配置。
/// 如果未提供配置文件路径，将尝试在可执行文件同目录下查找与包名同名的YAML配置文件。
/// 如果提供了端口参数，将覆盖配置文件中的端口设置。
///
/// ## 参数
/// * `path` - 可选的配置文件路径，如果为None则使用当前程序所在的目录
/// * `port` - 可选的端口号，如果提供将覆盖配置文件中的端口设置
///
/// ## 返回值
/// 返回解析后的Settings实例
///
/// ## Panics
/// 当配置文件读取失败或解析失败时会触发panic
pub fn init_settings(path: Option<String>, port: Option<u16>) {
    let mut settings = get_settings::<Settings>(path);

    info!("检查命令行是否指定了一些参数，如果有，则以命令行指定的参数为准...");
    // 如果命令行指定了端口，则使用命令行指定的端口
    if port.is_some() {
        settings.web_server.port = port;
    }

    info!("检查配置是否符合规范...");
    if settings.db.url.is_empty() {
        panic!("尚未配置db.url(数据库连接字符串)项");
    }

    SETTINGS.set(settings).expect("无法设置配置信息");
}
