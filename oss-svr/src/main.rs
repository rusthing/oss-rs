use clap::Parser;
use idworker::init_id_worker;
use log::info;
use oss_svr::config::{init_app_config, APP_CONFIG};
use oss_svr::db::migrate;
use oss_svr::web_service_config::web_service_config;
use robotech::db::init_db;
use robotech::env::init_env;
use robotech::log::init_log;
use robotech::web::start_web_server;

/// oss - 对象存储服务
///
/// SUMMARY: oss-svr 是一个对象存储服务，提供文件上传、下载、管理等功能。
/// 该服务支持多种存储后端，包括本地存储和云存储服务。
/// 通过 RESTful API 接口提供服务，支持 HTTP 和 HTTPS 协议。
/// 支持基于角色的访问控制(RBAC)和细粒度权限管理。
/// 提供完善的日志记录和监控功能，便于运维和问题排查。
///
#[derive(Parser, Debug)]
// 命令行参数使用定义
// version: 命令行添加 -V/--version参数可以查看版本信息
// about: --help命令第一行显示文档注释的内容
// long_about = None: 只显示文档注释的第一行(包括about的和arg的)
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
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("程序正在启动……");

    info!("初始化环境变量...");
    init_env();

    info!("初始化日志系统...");
    init_log()?;

    info!("解析命令行参数...");
    let args = Args::parse();

    info!("初始化设置选项...");
    init_app_config(args.config_file, args.port);

    // 升级数据库版本...
    migrate().await.expect("升级数据库版本失败");

    // 初始化ID生成器...
    let id_worker_config = APP_CONFIG.get().unwrap().id_worker.clone();
    init_id_worker(id_worker_config);

    // 初始化数据库连接
    init_db(APP_CONFIG.get().unwrap().db.clone()).await;

    // 启动Web服务
    let web_server_config = APP_CONFIG.get().unwrap().web_server.clone();
    start_web_server(web_server_config, web_service_config).await;

    info!("退出程序");
    Ok(())
}
