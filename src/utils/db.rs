use crate::settings::SETTINGS;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    let db_config = SETTINGS.get().unwrap().db.clone();

    let mut opt = ConnectOptions::new(db_config.url);
    opt.sqlx_logging_level(log::LevelFilter::Trace);
    // 连接数据库
    Database::connect(opt).await.expect("连接数据库失败")
}
