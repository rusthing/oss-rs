use clap::Parser;
use log::{debug, info};
use oss_rs::config::init_config;
use oss_rs::env::init_env;
use oss_rs::id_worker::init_id_worker;
use oss_rs::log::init_log;
use oss_rs::web_server::WebServer;

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
    init_env();

    init_log()?;

    info!("程序正在启动……");

    debug!("解析命令行参数...");
    let args = Args::parse();

    debug!("加载配置文件...");
    init_config(args.config_file, args.port);

    debug!("初始化ID生成器...");
    init_id_worker();

    WebServer::new().await.run().await;

    info!("退出程序");
    Ok(())
}
