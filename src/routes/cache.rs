use crate::util::durations;
use actix_web::http::Method;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use log::info;
use rovkit::filekit;
use std::fs;
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
        _ => Ok(HttpResponse::MethodNotAllowed().body("Unsupported method")),
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

    let path = req_path.trim_start_matches('/');
    Some(Path::new(CACHE_DIR).join(path))
}

async fn handle_get(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 GET 请求: {}:", path);

    let full_path = get_file_path(path).expect("Invalid path");

    if full_path.exists() || full_path.is_file() {
        let data = filekit::read_data(&full_path)?;
        return Ok(HttpResponse::Ok().body(data));
    }

    Ok(HttpResponse::NotFound().body("File not found"))
}

async fn handle_put(path: &str, data: &[u8]) -> Result<HttpResponse, std::io::Error> {
    info!("处理 PUT 请求: {}, data.len = {}", path, data.len());

    let full_path = get_file_path(path).expect("Invalid path");

    // 确保目录存在
    filekit::create_parent_dir(&full_path)?;
    let t = Utc::now().timestamp();
    let temp = format!("{}.{}.t", full_path.to_str().unwrap(), t);

    filekit::write_data(&full_path, data)?;
    fs::rename(&temp, &full_path)?;

    Ok(HttpResponse::Ok().body("File written successfully"))
}

async fn handle_delete(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 DELETE 请求: {}", path);

    let full_path = get_file_path(path).expect("Invalid path");

    if full_path.exists() || full_path.is_file() {
        filekit::remove_file(&full_path)?;
    }

    Ok(HttpResponse::Ok().finish())
}

async fn handle_head(path: &str) -> Result<HttpResponse, std::io::Error> {
    info!("处理 HEAD 请求: {}", path);

    let full_path = get_file_path(path).expect("Invalid path");

    if full_path.exists() || full_path.is_file() {
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::NotFound().body("File not found"))
}
