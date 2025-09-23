use bytesize::ByteSize;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::{env, fs};

/// 全局配置
pub static CONFIG: OnceLock<Config> = OnceLock::new();

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// oss
    #[serde(default = "oss_default")]
    pub oss: OssConfig,
    /// db
    #[serde(default = "db_default")]
    pub db: DbConfig,
    /// Web服务器
    #[serde(default = "web_server_default")]
    pub web_server: WebServerConfig,
    /// id_worker
    #[serde(default = "id_worker_default")]
    pub id_worker: IdWorkerConfig,
}

fn config_default() -> Config {
    Config {
        db: db_default(),
        web_server: web_server_default(),
        oss: oss_default(),
        id_worker: id_worker_default(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct OssConfig {
    /// 文件根目录
    #[serde(default = "file_root_dir_default")]
    pub file_root_dir: String,
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
fn oss_default() -> OssConfig {
    OssConfig {
        file_root_dir: file_root_dir_default(),
        upload_file_limit_size: upload_file_limit_size_default(),
        upload_buffer_size: upload_buffer_size_default(),
        download_buffer_size: download_buffer_size_default(),
    }
}

fn file_root_dir_default() -> String {
    "oss".to_string()
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DbConfig {
    #[serde(default = "url_default")]
    pub url: String,
}

fn url_default() -> String {
    "".to_string()
}

fn db_default() -> DbConfig {
    DbConfig { url: url_default() }
}

fn web_server_default() -> WebServerConfig {
    WebServerConfig {
        bind: bind_default(),
        port: port_default(),
        upload_file_limit_size: upload_file_limit_size_default(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct WebServerConfig {
    /// 绑定的IP地址
    #[serde(default = "bind_default")]
    pub bind: Vec<String>,
    /// Web服务器的端口号
    #[serde(default = "port_default")]
    pub port: Option<u16>,
    /// 上传文件限制的大小
    #[serde(default = "upload_file_limit_size_default")]
    pub upload_file_limit_size: ByteSize,
}

fn bind_default() -> Vec<String> {
    vec![String::from("0.0.0.0")]
}

fn port_default() -> Option<u16> {
    Some(9840)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct IdWorkerConfig {
    /// 基准时间(基于1ms为1个单位)
    #[serde(default = "epoch_default")]
    pub epoch: u64,
    /// 数据中心ID
    #[serde(default = "data_center_default")]
    pub data_center: u8,
    /// 数据中心ID位数
    #[serde(default = "data_center_bits_default")]
    pub data_center_bits: u8,
    /// 节点ID
    #[serde(default = "node_default")]
    pub node: u8,
    /// 节点ID位数
    #[serde(default = "node_bits_default")]
    pub node_bits: u8,
}

fn id_worker_default() -> IdWorkerConfig {
    IdWorkerConfig {
        epoch: epoch_default(),
        data_center: data_center_default(),
        data_center_bits: data_center_bits_default(),
        node: node_default(),
        node_bits: node_bits_default(),
    }
}

fn epoch_default() -> u64 {
    1758107692220
}
fn data_center_default() -> u8 {
    0
}
fn data_center_bits_default() -> u8 {
    0
}
fn node_default() -> u8 {
    0
}

fn node_bits_default() -> u8 {
    3
}

impl Config {
    /// 创建新的配置实例
    ///
    /// 该函数用于初始化应用程序配置，支持通过配置文件路径和端口参数来定制配置。
    /// 如果未提供配置文件路径，将尝试在可执行文件同目录下查找与包名同名的YAML配置文件。
    /// 如果提供了端口参数，将覆盖配置文件中的端口设置。
    ///
    /// # 参数
    /// * `path` - 可选的配置文件路径，如果为None则使用当前程序所在的目录
    /// * `port` - 可选的端口号，如果提供将覆盖配置文件中的端口设置
    ///
    /// # 返回值
    /// 返回解析后的Config实例
    ///
    /// # Panics
    /// 当配置文件读取失败或解析失败时会触发panic
    pub fn new(path: Option<String>, port: Option<u16>) -> Self {
        // 读取配置文件内容到字符串
        let content = if path.is_some() {
            // 如果已指定配置文件路径
            Some(fs::read_to_string(path.unwrap()).expect("读取配置文件失败"))
        } else {
            // 如果未指定配置文件路径
            let mut exe_file_path = env::current_exe().expect("获取可执行文件路径失败");
            let config_file_name = concat!(env!("CARGO_PKG_NAME"), ".yml");
            exe_file_path.pop(); // 移除可执行文件名
            let config_file_path = exe_file_path.join(&config_file_name);
            match fs::read_to_string(config_file_path) {
                Ok(content) => Some(content),
                Err(e) => {
                    warn!(
                        "读取配置文件内容出错，可能是配置文件不存在，将使用默认配置: \n{}",
                        e
                    );
                    None
                }
            }
        };

        // 解析配置文件
        let mut config: Config = if content.is_some() {
            // 如果已正常获取配置文件内容
            serde_yaml::from_str(content.unwrap().as_str()).expect("解析配置文件失败")
        } else {
            // 如果未正常获取配置文件内容
            config_default()
        };

        // 如果web_server未设置上传文件限制的大小，则以oss里面设置的为准
        if config.web_server.upload_file_limit_size == upload_file_limit_size_default()
            && config.oss.upload_file_limit_size != upload_file_limit_size_default()
        {
            config.web_server.upload_file_limit_size = config.oss.upload_file_limit_size;
        }

        // 如果命令行指定了端口，则使用命令行指定的端口
        if port.is_some() {
            config.web_server.port = port;
        }

        info!("检查配置是否符合规范");
        if config.db.url.is_empty() {
            panic!("配置文件中没有配置db.url(数据库连接字符串)");
        }

        config
    }
}
