use crate::api::api_config::api_config;
use crate::settings::SETTINGS;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;

pub struct WebServer {
    pub server: Server,
}

impl WebServer {
    pub async fn new() -> Self {
        let web_server_config = SETTINGS.get().unwrap().web_server.clone();
        info!("创建Web服务器({:?})并运行...", web_server_config);

        let port = web_server_config.port.unwrap();
        let mut server =
            HttpServer::new(move || App::new().wrap(Logger::default()).configure(api_config));

        // 绑定IP地址
        for bind in web_server_config.bind {
            server = server.bind((bind, port)).unwrap();
        }

        // 启动服务器
        let server = server.run();

        Self { server }
    }

    pub async fn run(self) {
        self.server.await.expect("服务器启动失败");
    }
}
