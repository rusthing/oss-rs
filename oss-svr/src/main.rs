use anyhow::anyhow;
use clap::Parser;
use config::Value;
use idworker::setup_id_worker;
use oss_svr::config::{setup_oss_config, AppConfig};
use robotech;
use robotech::app::{wait_app_exit, AppWatcher};
use robotech::dao::init_dao;
use robotech::db::setup_db_conn;
use robotech::env::init_env;
use robotech::log::LogWatcher;
use robotech::macros::db_migrate;
use robotech::signal::SignalManager;
use robotech::web::{setup_web_server, stop_web_service};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// oss - 对象存储服务
///
/// SUMMARY: oss-svr 是一个对象存储服务，提供文件上传、下载、管理等功能。
/// 该服务支持多种存储后端，包括本地存储和云存储服务。
/// 通过 RESTful API 接口提供服务，支持 HTTP 和 HTTPS 协议。
/// 支持基于角色的访问控制(RBAC)和细粒度权限管理。
/// 提供完善的日志记录和监控功能，便于运维和问题排查。
///
#[derive(Parser, Debug, Clone)]
// 命令行参数使用定义
// version: 命令行添加 -V/--version参数可以查看版本信息
// about: --help命令第一行显示文档注释的内容
// long_about = None: 只显示文档注释的第一行(包括about的和arg的)
// help_template: 帮助模板
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version,
    about,
    help_template = "{name} v{version} - {about}\n\nAUTHOR: {author}\n\nUSAGE: {usage}\n\nOPTIONS:\n{options}"
)]
struct Args {
    /// 配置文件的路径
    #[arg(short, long)]
    config_file: Option<String>,

    /// Web服务器的端口号
    #[arg(short, long)]
    port: Option<u16>,

    /// 监听信号，支持指令如下:
    /// * `start` - 默认值，先发送`SIGCONT`信号(kill -0)，检查程序是否已运行(如果程序已运行，会报错)，然后启动程序
    /// * `restart` - 先发送`SIGTERM`信号(kill -15)，如果旧程序已运行，收到信号后会停止运行，然后启动新程序
    /// * `stop`/`s` - 发送`SIGTERM`信号(kill -15)，用于终止程序，优雅退出
    /// * `kill`/`k` - 发送`SIGKILL`信号(kill -9)，用于强制终止程序
    #[arg(
        short,
        long,
        default_value = "start",
        long_help = r#"监听信号，支持指令如下:
    start - 默认值，先发送 SIGCONT 信号(kill -0)，检查程序是否已运行(如果程序已运行，会报错)，然后启动程序
    restart - 先发送 SIGTERM 信号(kill -15)，如果旧程序已运行，收到信号后会停止运行，然后启动新程序
    stop/s - 发送 SIGTERM 信号(kill -15)，用于终止程序，优雅退出
    kill/k - 发送 SIGKILL 信号(kill -9)，用于强制终止程序"#
    )]
    signal: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let Args {
        signal,
        config_file: config_file_path,
        port,
    } = Args::parse();

    // 初始化环境变量;
    init_env()?;
    // 初始化日志系统
    let log_watcher = LogWatcher::new().await?;
    // 初始化数据访问层
    init_dao()?;

    // 初始化信号(_signal_manager变量将在程序优雅退出时释放，释放时删除pid文件)
    let (mut signal_manager, old_pid) = SignalManager::new(signal)?;

    let app_watcher: AppWatcher<AppConfig> = AppWatcher::new(
        config_file_path,
        log_watcher.config_changed_tx.clone(),
        move |app_config: Arc<AppConfig>, changed| async move {
            let changed = Some(changed);
            setup(&app_config, &changed, port, old_pid).await?;

            info!("重新加载配置成功");
            Ok(())
        },
    )
    .await?;

    let changed = None;
    setup(&app_watcher.app_config, &changed, port, old_pid).await?;

    // 监听系统信号与等待退出
    let signal_receiver = signal_manager.watch_signal()?;
    Ok(wait_app_exit(signal_receiver, || async move {
        stop_web_service().await.expect("无法停止旧的Web服务");
        Ok(())
    })
    .await?)
}

/// # 初始化或更新应用配置
/// ## 参数
/// * `app_config` - 应用配置的Arc智能指针，用于访问和修改配置
/// * `changed` - 一个可选的HashMap，用于存储配置中发生改变的键值对
/// * `port` - 一个可选的u16值，指定Web服务器监听的端口。如果未指定，则使用配置文件中的设置或默认值。
/// * `old_pid` - 一个可选的i32值，代表旧进程ID，用于在重启时清理资源等操作。
async fn setup(
    app_config: &Arc<AppConfig>,
    changed: &Option<HashMap<String, Value>>,
    port: Option<u16>,
    old_pid: Option<u32>,
) -> Result<(), anyhow::Error> {
    // 初始化或更新ID生成器...
    setup_id_worker(app_config.id_worker.clone(), &changed)?;
    // 初始化或更新oss配置...
    setup_oss_config(app_config.oss.clone(), &changed);
    // 初始化或更新数据库连接...
    setup_db_conn(app_config.db.clone(), &changed).await?;

    // 升级数据库版本...
    let db_url = app_config.db.get_url();
    db_migrate!(db_url);

    // 初始化或更新Web服务器...
    setup_web_server(app_config.web.clone(), port, old_pid, &changed).await?;

    Ok(())
}
