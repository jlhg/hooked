use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    github_webhook_secret: String,
    discord_webhook_url: String
}

pub fn gen_default_config() -> Config {
    Config {
        github_webhook_secret: String::new(),
        discord_webhook_url: String::new()
    }
}

pub fn gen_default_config_file(out_path: &str) -> Result<(), Box<dyn Error>> {
    let config = gen_default_config();
    let toml = toml::to_string(&config)?;
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out_path)?;
    f.write_all(toml.as_bytes())?;
    Ok(())
}
