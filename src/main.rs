mod routes;

use actix_web::{web, App, HttpServer};
use routes::index::{hello, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
