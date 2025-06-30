use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
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
    pub s3: S3Config,
}

pub fn load_config() -> Config {
    let dev = "Config.toml";
    let prod = "Config.prod.toml";
    let mut f = dev;
    if rovkit::iokit::path_exists(prod) {
        f = prod;
    }
    let s = rovkit::iokit::read_file_to_string(f).unwrap();
    toml::from_str(&s).expect("无法解析配置文件")
}
