use clap::ArgAction;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None, disable_help_flag = true)]
pub struct Config {
    /// Path to the log file.
    #[arg(short, long = "log", env, default_value = "log/app.log")]
    pub log_file_path: String,

    /// The IP address where the server is hosted.
    #[arg(short, long, env = "HOOKED_HOST", default_value = "0.0.0.0")]
    pub host: String,

    /// The port number where the server is listening.
    #[arg(short, long, env = "HOOKED_PORT", default_value_t = 3000)]
    pub port: u16,

    /// Path to the build entry script.
    #[arg(long, env)]
    pub build_entry_script_path: String,

    /// The token to verify the incoming GitHub webhook messages. See
    /// [Creating webhooks - GitHub
    /// Docs](https://docs.github.com/en/webhooks/using-webhooks/creating-webhooks)
    /// for creating a webhook and setting the secret token.
    #[arg(long, env, value_parser = parse_secret_or_env)]
    pub github_webhook_secret: String,

    /// The Git branch name to watch.
    #[arg(long, env)]
    pub github_watch_push_branch: String,

    /// The Discord webhook URL to send the notification to Discord
    /// channel. See [Intro to Webhooks –
    /// Discord](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks)
    /// for creating a Discord webhook URL.
    #[arg(long, env, value_parser = parse_secret_or_env)]
    pub discord_webhook_url: String,

    #[arg(short = '?', long, action = ArgAction::Help, help = "Print help")]
    pub help: (),
}

fn parse_secret_or_env(value: &str) -> Result<String, String> {
    if value.starts_with("/run/secrets/") {
        fs::read_to_string(value)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read secret file: {}", e))
    } else {
        Ok(value.to_string())
    }
}

impl Config {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
