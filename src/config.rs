use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("Config.toml")
        .expect("无法读取配置文件");
    
    toml::from_str(&config_str)
        .expect("无法解析配置文件")
}