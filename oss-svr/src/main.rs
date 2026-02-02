use clap::Parser;
use idworker::init_id_worker;
use log::info;
use oss_svr::config::AppConfig;
use oss_svr::db::migrate;
use oss_svr::global::set_app_config;
use oss_svr::web_service_config::web_service_config;
use robotech::config::build_app_config;
use robotech::db::init_db;
use robotech::env::init_env;
use robotech::log::init_log;
use robotech::signal::parse_and_handle_signal_args;
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

    /// 监听信号
    /// start: 启动程序
    /// restart: 重启程序
    /// r,reload: 重新加载配置文件
    /// q,quit: 中断退出程序(Ctrl+C)
    /// s,stop: 优雅停止程序(kill -15)
    /// k,kill: 暴力停止程序(kill -9)
    #[arg(short, long)]
    signal: Option<String>,
}

#[tokio::main]
async fn main() {
    info!("程序正在启动……");

    info!("初始化环境变量...");
    init_env();

    info!("初始化日志系统...");
    init_log();

    info!("解析命令行参数...");
    let args = Args::parse();

    // 解析与处理信号参数(此变量将在程序优雅退出时释放，释放时删除pid文件)
    let _pid_file_guard = parse_and_handle_signal_args(args.signal);

    info!("构建配置信息...");
    let app_config: AppConfig = build_app_config(args.config_file);
    set_app_config(app_config.clone());

    // 升级数据库版本...
    migrate(app_config.db.clone())
        .await
        .expect("升级数据库版本失败");

    // 初始化ID生成器...
    init_id_worker(app_config.id_worker);

    // 初始化数据库连接
    init_db(app_config.db).await;

    // 启动Web服务
    start_web_server(app_config.web_server, web_service_config, args.port).await;

    info!("退出程序");
}
