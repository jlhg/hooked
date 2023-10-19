use crate::arg::parse_args;
use crate::config::gen_default_config_file;
use crate::logger::setup_logger;
use log::{error};
use std::process::exit;

mod arg;
mod config;
mod logger;

fn main() {
    if let Err(e) = setup_logger("hooked.log") {
        eprintln!("Failed to set up logger: {}", e);
        exit(1);
    }

    let cmd = parse_args();
    match cmd.get_matches().subcommand() {
        Some(("init", _sub_matches)) => {
            let config_file_path = String::from("config.toml");
            if let Err(e) = gen_default_config_file(&config_file_path) {
                error!("{}: {}", config_file_path, e);
                exit(1);
            }
        }
        Some(("start", _sub_matches)) => {
            // TODO
            println!("start a server");
        }
        _ => unreachable!()
    }
}
