use clap::Parser;
use idworker::init_id_worker;
use log::info;
use oss_svr::db::migrate;
use oss_svr::env::init_env;
use oss_svr::log::init_log;
use oss_svr::settings::{init_settings, SETTINGS};
use oss_svr::web_service_config::web_service_config;
use robotech::db::init_db;
use robotech::web_server::start_web_server;

/// 网络监控工具
///
/// SUMMARY: 这是一个用于网络监控的工具，可以监控各种网络目标并提供指标收集功能
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
    init_settings(args.config_file, args.port);

    // 升级数据库版本...
    migrate().await.expect("升级数据库版本失败");

    // 初始化ID生成器...;
    let id_worker_settings = SETTINGS.get().unwrap().id_worker.clone();
    init_id_worker(id_worker_settings);

    // 初始化数据库连接
    init_db(SETTINGS.get().unwrap().db.clone()).await;

    // 启动Web服务
    let web_server_settings = SETTINGS.get().unwrap().web_server.clone();
    start_web_server(web_server_settings, web_service_config).await;

    info!("退出程序");
    Ok(())
}
