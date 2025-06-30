use crate::routes::cache_local::LocalCache;
use crate::util::durations;
use actix_web::http::Method;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use std::time::Instant;

/// 匹配所有其他路径
pub async fn cache_router_handler(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let method = req.method().clone();
    let path = req.path();
    let start = Instant::now();
    info!("{method} {path} >>>");

    let ret = match method {
        Method::GET => handle_get(path).await,
        Method::HEAD => handle_head(path).await,
        Method::PUT => handle_put(path, &body).await,
        Method::DELETE => handle_delete(path).await,
        _ => Ok(HttpResponse::MethodNotAllowed().body("Unsupported method")),
    };

    info!(
        "{method} {path} <<< time = {}",
        durations::display(start.elapsed()),
    );
    ret
}
async fn handle_get(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 GET 请求: {}:", path);

    let local = LocalCache::new(path.to_string());
    if local.exist() {
        return match local.read() {
            Ok(data) => Ok(HttpResponse::Ok().body(data)),
            Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
        };
    }

    Ok(HttpResponse::NotFound().body("File not found"))
}

async fn handle_put(path: &str, data: &[u8]) -> Result<HttpResponse, std::io::Error> {
    info!("处理 PUT 请求: {}, data.len = {}", path, data.len());

    let local = LocalCache::new(path.to_string());
    match local.write(data) {
        Ok(_) => Ok(HttpResponse::Ok().body("Write file successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Write file failed")),
    }
}

async fn handle_delete(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 DELETE 请求: {}", path);

    let local = LocalCache::new(path.to_string());
    local.delete().expect("Delete file failed");

    Ok(HttpResponse::Ok().finish())
}

async fn handle_head(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 HEAD 请求: {}", path);

    let local = LocalCache::new(path.to_string());

    if local.exist() {
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::NotFound().body("File not found"))
}
