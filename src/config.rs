use rovkit::jsonkit;
use rovkit::singlekit::Single;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3Config {
    pub region: String,
    pub endpoint: String,
    pub accessKey: String,
    pub secretKey: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub s3: S3Config,
}

pub static GLOBAL_CONFIG: Single<Config> = Single::new();

pub fn load_config() {
    let config = do_load();
    log::info!("config: {}", jsonkit::to_pretty_json(&config).unwrap());

    GLOBAL_CONFIG.get_or_init(|| config);
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
