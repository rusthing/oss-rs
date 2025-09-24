use std::{env, fs};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_log() -> Result<(), std::io::Error> {
    // 自定义日志格式，包含 "at" 文本
    let console_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(std::io::stdout);

    // 输出到文件
    let mut exe_file_path = env::current_exe().expect("获取可执行文件路径失败");
    exe_file_path.pop(); // 移除可执行文件名
    let log_dir = exe_file_path.join("log");
    fs::create_dir_all(log_dir.as_path())?;
    let file_appender = tracing_appender::rolling::hourly(log_dir, "oss-rs.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().json().with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(console_layer) // 控制台输出
        .with(file_layer) // 文件输出
        .init();
    tracing::info!("日志初始化成功");

    Ok(())
}
