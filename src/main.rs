use crate::arg::parse_args;
use crate::config::{gen_default_config, read_config_file};
use crate::logger::setup_logger;
use crate::web::app::start_server;

mod arg;
mod config;
mod crypto;
mod logger;
mod web;

#[tokio::main]
async fn main() {
    const LOG_FILE_PATH: &str = "hooked.log";
    const CONFIG_FILE_PATH: &str = "config.toml";

    let _guard = setup_logger(LOG_FILE_PATH).expect("setting up logger failed");

    let cmd = parse_args();
    match cmd.get_matches().subcommand() {
        Some(("init", _sub_matches)) => {
            let cfg = gen_default_config();
            cfg.write_to_file(&CONFIG_FILE_PATH)
                .expect("writing the default config file failed");
        }
        Some(("server", _sub_matches)) => match read_config_file(&CONFIG_FILE_PATH) {
            Ok(cfg) => {
                println!("starting a web server");
                start_server(cfg)
                    .await
                    .expect("starting a web server failed");
            }
            Err(e) => {
                panic!("reading the config file failed: {}", e);
            }
        },
        _ => unreachable!(),
    }
}
