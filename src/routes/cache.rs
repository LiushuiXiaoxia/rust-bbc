use crate::util::durations;
use actix_web::http::Method;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{info, warn};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// 匹配所有其他路径
pub async fn cache_handler(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let method = req.method().clone();
    let path = req.path();
    let start = Instant::now();
    info!("{method} {path} >>>");

    let ret = match method {
        Method::GET => handle_get(path).await,
        Method::HEAD => handle_head(path).await,
        Method::PUT => handle_put(path, &body).await,
        Method::DELETE => handle_delete(path).await,
        _ => HttpResponse::MethodNotAllowed().body("Unsupported method"),
    };

    info!(
        "{method} {path} <<< time = {}",
        durations::display(start.elapsed()),
    );
    ret
}
const CACHE_DIR: &str = "cache";

/// 根目录配置
fn get_file_path(req_path: &str) -> Option<PathBuf> {
    if req_path.contains("..") {
        return None;
    }
    let relative_path = req_path.trim_start_matches('/');
    Some(Path::new(CACHE_DIR).join(relative_path))
}

async fn handle_get(path: &str) -> HttpResponse {
    info!("处理 GET 请求: {}:", path);
    let base_dir = Path::new(CACHE_DIR);
    let relative_path = &path[1..]; // 去掉开头的 `/`
    let full_path = base_dir.join(relative_path);

    if full_path.exists() || full_path.is_file() {
        match fs::read(&full_path) {
            Ok(data) => HttpResponse::Ok().body(data),
            Err(_) => HttpResponse::NotFound().body("Failed to read file"),
        }
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

async fn handle_put(path: &str, data: &[u8]) -> HttpResponse {
    info!("处理 PUT 请求: {}, data.len = {}", path, data.len());

    let full_path = match get_file_path(path) {
        Some(p) => p,
        None => return HttpResponse::Forbidden().body("Invalid path"),
    };

    // 确保目录存在
    if let Some(parent) = full_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            let s = format!("Failed to create dir: {}", e);
            return HttpResponse::InternalServerError().body(s);
        }
    }

    match fs::File::create(&full_path).and_then(|mut f| f.write_all(data)) {
        Ok(_) => HttpResponse::Ok().body("File written successfully"),
        Err(e) => {
            let string = format!("Failed to write file: {}", e);
            HttpResponse::InternalServerError().body(string)
        }
    }
}

async fn handle_delete(path: &str) -> HttpResponse {
    info!("处理 DELETE 请求: {}", path);

    let base_dir = Path::new(CACHE_DIR);
    let relative_path = &path[1..]; // 去掉开头的 `/`
    let full_path = base_dir.join(relative_path);

    if full_path.exists() || full_path.is_file() {
        fs::remove_file(&full_path).unwrap_or_else(|_| warn!("无法删除文件"));
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

async fn handle_head(path: &str) -> HttpResponse {
    info!("处理 HEAD 请求: {}", path);

    let base_dir = Path::new(CACHE_DIR);
    let relative_path = &path[1..]; // 去掉开头的 `/`
    let full_path = base_dir.join(relative_path);

    if full_path.exists() || full_path.is_file() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}
