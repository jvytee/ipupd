use serde::Deserialize;
use std::fs::File;
use std::io::{prelude::*, Error, ErrorKind, Result};

#[derive(Deserialize)]
pub struct Config {
    pub interface: String,
    pub domain: String,
    pub url: String,
    pub basic_auth: Option<Auth>,
    pub query: Query
}

#[derive(Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub ipv4: String,
    pub ipv6: String
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Config> {
        let mut file = File::open(filename)?;
        let mut content = String::new();

        return if let Ok(_bytes) = file.read_to_string(&mut content) {
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Could not parse configuration data",
            ))
        };
    }
}
