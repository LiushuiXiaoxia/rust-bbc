mod config;
mod middleware;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use config::load_config;
use env_logger::Env;
use middleware::RequestHandler;
use routes::index::{health_check, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = load_config();
    let addr = format!("http://{}:{}", config.server.host, config.server.port);
    println!("Server starting at {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestHandler::new())
            .service(hello)
            .service(health_check)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
