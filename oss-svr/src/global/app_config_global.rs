use crate::app::AppConfig;
use std::sync::RwLock;

static APP_CONFIG: RwLock<Option<AppConfig>> = RwLock::new(None);

/// 获取当前配置的只读访问
pub fn get_app_config() -> AppConfig {
    let read_lock = APP_CONFIG.read().expect("Failed to acquire lock");
    read_lock.clone().unwrap()
}

/// 设置配置
pub fn set_app_config(config: AppConfig) {
    let mut write_lock = APP_CONFIG.write().expect("Failed to acquire lock");
    *write_lock = Some(config);
}
