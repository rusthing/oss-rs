use sea_orm::{Database, DatabaseConnection};

pub async fn get_conn() -> DatabaseConnection {
    // 连接数据库
    Database::connect("postgres://oss:oss@127.0.0.1/oss")
        .await
        .expect("连接数据库失败")
}
