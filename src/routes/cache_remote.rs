use crate::config;
use aws_config;
use aws_config::Region;
use aws_sdk_s3::config::{Builder, Credentials};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use log::{error, info};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub async fn create_s3_client() -> Client {
    let config = config::config();
    let base = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let mut builder = Builder::from(&base)
        .endpoint_url(config.s3.endpoint.clone())
        .credentials_provider(Credentials::new(
            config.s3.access_key.clone(),
            config.s3.secret_key.clone(),
            None,
            None,
            "bbc",
        ));
    if !config.s3.region.is_empty() {
        builder = builder.region(Region::new(config.s3.region.clone()));
    }
    Client::from_conf(builder.build())
}

static S3_CLIENT: OnceLock<Client> = OnceLock::new();
pub async fn s3_client() -> &'static Client {
    if S3_CLIENT.get().is_none() {
        let client = create_s3_client().await;
        S3_CLIENT.set(client).unwrap();
    }
    S3_CLIENT.get().unwrap()
}

pub struct RemoteCache {
    bucket: String,
    pub key: String,
    pub file: PathBuf,
}

impl RemoteCache {
    pub fn new(key: String, file: PathBuf) -> Self {
        let bucket = config::config().s3.bucket.clone();
        Self { bucket, key, file }
    }

    pub async fn exist(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let ret = s3_client()
            .await
            .head_object()
            .bucket(self.bucket.as_str())
            .key(self.key.as_str())
            .send()
            .await;
        match ret {
            Ok(_) => {
                info!("remote exist, key = {}, ret = true", self.key);

                Ok(true)
            }
            Err(e) => {
                if e.raw_response().is_some()
                    && e.raw_response().unwrap().status().is_client_error()
                {
                    info!("remote exist, key = {}, ret = false", self.key);
                    return Ok(false);
                }
                error!("remote exist, key = {}, e = {}", self.key, e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn delete(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.exist().await.is_ok() {
            let ret = s3_client()
                .await
                .delete_object()
                .bucket(self.bucket.as_str())
                .key(self.key.as_str())
                .send()
                .await;

            return match ret {
                Ok(_) => {
                    info!("remote delete, key = {}, ret = true", self.key);
                    Ok(())
                }
                Err(e) => {
                    error!("remote delete, key = {}, e = {}", self.key, e);
                    Err(Box::new(e))
                }
            };
        }

        info!("remote delete, key = {}, not exist", self.key);
        Ok(())
    }

    pub async fn write(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let data = fs::read(self.file.as_path())?;
        let body = ByteStream::from(data);

        let ret = s3_client()
            .await
            .put_object()
            .bucket(self.bucket.as_str())
            .key(self.key.as_str())
            .body(body)
            .send()
            .await;

        match ret {
            Ok(_) => {
                info!("remote write, key = {}, ret = true", self.key);
                Ok(())
            }
            Err(e) => {
                info!("remote write, key = {}, ret = false ,e = {}", self.key, e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn read(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ret = s3_client()
            .await
            .get_object()
            .bucket(self.bucket.as_str())
            .key(self.key.as_str())
            .send()
            .await;
        match ret {
            Ok(resp) => {
                let data = resp.body.collect().await?.into_bytes().to_vec();
                fs::write(&self.file, data)?;

                info!("remote read, key = {}, success", self.key);
                Ok(())
            }
            Err(error) => {
                error!("remote read, key = {}, error = {}", self.key, error);
                Err(Box::new(error))
            }
        }
    }
}
