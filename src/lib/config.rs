use std::net::Ipv4Addr;
use std::path::Path;

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub ip: Ipv4Addr,
    pub upload_dir: PathBuf,
    pub db_file: PathBuf,
}

pub async fn load_config_file(path: &Path) -> Config {
    let bytes = match async_std::fs::read(path).await {
        Ok(bytes) => bytes,
        Err(error) => {
            panic!(
                "Could not read config file {path:?}: {error:?}",
                path = path,
                error = error
            )
        }
    };

    match toml::de::from_slice(&bytes) {
        Ok(config) => config,
        Err(error) => {
            panic!(
                "Could not parse config file {path:?}: {error:?}",
                path = path,
                error = error
            )
        }
    }
}
