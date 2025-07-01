use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub payload: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub s3: S3Config,
}

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();
pub fn config() -> &'static Config {
    GLOBAL_CONFIG.get_or_init(|| do_load())
}
fn do_load() -> Config {
    let dev = "Config.toml";
    let prod = "Config.prod.toml";
    let mut f = dev;
    if rovkit::iokit::path_exists(prod) {
        f = prod;
    }
    let s = rovkit::iokit::read_file_to_string(f).unwrap();
    toml::from_str(&s).expect("无法解析配置文件")
}
