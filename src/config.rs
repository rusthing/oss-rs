use log::warn;
use serde::{Deserialize, Serialize};
use std::{env, fs};

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Web服务器
    #[serde(default = "web_server_default")]
    pub web_server: WebServerConfig,
}

fn web_server_default() -> WebServerConfig {
    WebServerConfig {
        bind: bind_default(),
        port: port_default(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebServerConfig {
    /// 绑定的IP地址
    #[serde(default = "bind_default")]
    pub bind: Vec<String>,
    /// Web服务器的端口号
    #[serde(default = "port_default")]
    pub port: Option<u16>,
}

fn bind_default() -> Vec<String> {
    vec![String::from("0.0.0.0")]
}

fn port_default() -> Option<u16> {
    Some(9840)
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
            Self {
                web_server: WebServerConfig {
                    bind: bind_default(),
                    port: port_default(),
                },
            }
        };

        // 如果命令行指定了端口，则使用命令行指定的端口
        if port.is_some() {
            config.web_server.port = port;
        }

        // info!("检查配置是否符合规范");


        config
    }
}
