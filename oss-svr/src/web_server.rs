use crate::settings::SETTINGS;
use crate::web_service_config::configure;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;

pub async fn start_web_server() {
    let web_server_config = SETTINGS.get().unwrap().web_server.clone();
    info!("创建Web服务器({:?})并运行...", web_server_config);

    let port = web_server_config.port.unwrap();
    let mut server =
        HttpServer::new(move || App::new().wrap(Logger::default()).configure(configure));

    // 绑定IP地址
    for bind in web_server_config.bind {
        server = server.bind((bind, port)).unwrap();
    }

    // 启动服务器
    server.run().await.expect("服务器启动失败");
}
