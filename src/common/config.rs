use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub teloxide_token: String,

    pub postgres_host: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_dbname: String,
}

impl Config {
    pub fn from_toml(filename: &str) -> Config {
        let contents = fs::read_to_string(filename).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();

        config
    }
}
