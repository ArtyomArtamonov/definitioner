use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub teloxide_token: String,
}

impl Config {
    pub fn from_toml(filename: &str) -> Config {
        let contents = fs::read_to_string(filename).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();

        config
    }
}
