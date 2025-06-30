mod config;
mod routes;
mod util;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::load_config;
use env_logger::Env;
use log::info;
use routes::cache::cache_handler;
use routes::index::{health_check, hello};
use rovkit::jsonkit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = load_config();
    info!("config: {}", jsonkit::to_pretty_json(&config)?);

    let addr = format!("http://{}:{}", config.server.host, config.server.port);
    println!("Server starting at {}", addr);

    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(500 * 1024 * 1024))
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
            .default_service(web::to(cache_handler))
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
