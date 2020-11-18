use serde::Deserialize;
use std::fs::File;
use std::io::{
    prelude::*,
    Error,
    ErrorKind,
    Result
};

#[derive(Deserialize)]
pub struct Config {
    pub interface: String,
    pub domain: String,
    pub url: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Config> {
        let mut file = File::open(filename)?;
        let mut content = String::new();

        return if let Ok(_bytes) = file.read_to_string(&mut content) {
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Could not parse configuration data"))
        }
    }
}
