use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

/// 全局配置
pub static ENV: OnceLock<Env> = OnceLock::new();

#[derive(Debug)]
pub struct Env {
    pub app_dir: PathBuf,
    pub app_file_name: String,
}

/// 初始化环境变量
pub fn init_env() {
    let mut app_file_path = env::current_exe().expect("获取应用程序路径失败");
    let app_file_name = app_file_path
        .file_name()
        .expect("获取应用程序文件名称失败")
        .to_string_lossy()
        .to_string();
    app_file_path.pop();

    let env = Env {
        app_dir: app_file_path,
        app_file_name,
    };

    ENV.set(env).expect("无法设置环境变量");
}
