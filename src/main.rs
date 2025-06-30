mod config;
mod routes;
mod util;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::load_config;
use env_logger::Env;
use routes::index::{health_check, hello};
use routes::router::cache_router_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    load_config();
    let config = config::GLOBAL_CONFIG.get().unwrap().lock().unwrap();
    let host = config.server.host.as_str();
    let port = config.server.port;

    let addr = format!("http://{}:{}", host, port);
    println!("Server starting at {}", addr);

    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(500 * 1024 * 1024))
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
            .default_service(web::to(cache_router_handler))
    })
    .bind((host, port))?
    .run()
    .await
}
