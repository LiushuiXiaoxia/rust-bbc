use chrono::Utc;
use log::info;
use rovkit::{filekit, iokit};
use std::fs;
use std::path::{Path, PathBuf};

pub struct LocalCache {
    pub key: String,
    pub file: PathBuf,
}

/// 根目录配置
fn gen_cache_path(key: &str) -> Option<PathBuf> {
    if key.contains("..") || key.ends_with("/") {
        return None;
    }

    let config = crate::config::GLOBAL_CONFIG.lock().unwrap();
    let root = config.cache.root.as_str();
    let path = key.trim_start_matches('/').trim_end_matches("/");
    let t = std::path::absolute(Path::new(root).join(path)).unwrap();

    let mut m = "00";
    if path.len() > 2 {
        m = &t.file_name().unwrap().to_str().unwrap()[0..2]
    }

    let f = t.parent().unwrap().join(m).join(t.file_name().unwrap());
    iokit::create_dir_all(f.parent().unwrap()).unwrap();
    Some(f)
}

impl LocalCache {
    pub fn new(key: String) -> Self {
        let file = gen_cache_path(key.as_str()).expect("Invalid path");
        Self { key, file }
    }

    pub fn find(&self) -> &PathBuf {
        &self.file
    }

    pub fn exist(&self) -> bool {
        self.file.exists() && self.file.is_file()
    }

    pub fn delete(&self) -> Result<(), std::io::Error> {
        if self.exist() {
            return fs::remove_file(self.file.clone());
        }
        Ok(())
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        info!("write cache {} to {}", self.key, self.file.display());

        let ts = Utc::now().timestamp();
        let tmp = format!("{}.{}.t", self.file.to_str().unwrap(), ts);

        filekit::write_data(&tmp, data)?;
        fs::rename(&tmp, &self.file)
    }

    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        info!("read cache {} from {}", self.key, self.file.display());
        fs::read(self.file.clone())
    }
}
