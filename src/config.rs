use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub build_entry_script_path: String,
    pub github_webhook_secret: String,
    pub github_watch_push_branch: String,
    pub discord_webhook_url: String,
}

impl Config {
    pub fn write_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(&self)?;
        let mut f = OpenOptions::new().create_new(true).write(true).open(path)?;
        f.write_all(toml.as_bytes())?;
        Ok(())
    }
}

pub fn gen_default_config() -> Config {
    Config {
        host: String::from("0.0.0.0"),
        port: 3000,
        build_entry_script_path: String::from("build.sh"),
        github_webhook_secret: String::new(),
        github_watch_push_branch: String::new(),
        discord_webhook_url: String::new(),
    }
}

pub fn read_config_file(path: &str) -> Result<Config, Box<dyn Error>> {
    let c = read_to_string(path)?;
    let config = toml::from_str(&c)?;
    Ok(config)
}
