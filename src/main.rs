mod config;
mod routes;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::load_config;
use env_logger::Env;
use routes::cache::cache_handler;
use routes::index::{health_check, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = load_config();
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
