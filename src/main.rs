mod config;
mod routes;
mod util;

use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use log::info;
use routes::index::{health_check, hello};
use routes::router::cache_router;
use rovkit::jsonkit;

fn server() -> Server {
    let config = crate::config::config();
    info!("config = {}", jsonkit::to_pretty_json(config).unwrap());

    let host = config.server.host.as_str();
    let port = config.server.port;
    let payload = config.server.payload;
    let addr = format!("{}://{}:{}", "http", host, port);
    println!("Server starting at {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(payload * 1024 * 1024))
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
            .default_service(web::to(cache_router))
    })
    .bind((host, port))
    .unwrap()
    .run()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    server().await
}
