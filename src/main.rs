mod routes;
mod config;

use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use routes::index::{hello, health_check};
use config::load_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let config = load_config();
    let addr = format!("http://{}:{}", config.server.host, config.server.port);
    println!("Server starting at {}", addr);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
