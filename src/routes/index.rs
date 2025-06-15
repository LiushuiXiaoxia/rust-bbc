use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Service is running"
    }))
}