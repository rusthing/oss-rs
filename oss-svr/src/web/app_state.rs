use crate::app::{AppConfig, OssConfig};
use arc_swap::ArcSwap;
use axum::extract::FromRef;
use std::sync::Arc;

// 定义应用全局 AppState
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<ArcSwap<AppConfig>>,
}

// 声明：如何从 AppState 中提取 DbConfig
impl FromRef<AppState> for Arc<ArcSwap<AppConfig>> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for OssConfig {
    fn from_ref(app_state: &AppState) -> Self {
        let config = app_state.config.load_full();
        config.oss.clone()
    }
}
