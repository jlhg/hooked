# hooked

A simple command-line CI/CD tool that triggers custom build and deployment commands when it receives the GitHub webhook messages. It also supports Discord notification (including the stdout and stderr outputs).

## Screenshots

![](doc/screenshot/github_webhook.png)
![](doc/screenshot/discord_notification.png)

## Installation

### Build from the source code

Clone the repository:

```txt
git clone https://github.com/jlhg/hooked.git
```

Run `cargo build`:

```txt
cd hooked
cargo build --release
```

You can find the executable file at `./target/release/hooked`.

## Usage

Create a new config file:

```txt
hooked init
```

Edit the config file. The available options are described below:

- `host`: The domain or IP address where the server is hosted.
- `port`: The port number where the server is listening.
- `build_entry_script_path`: Path to the build entry script.
- `github_webhook_secret`: The token to verify the incoming GitHub webhook messages. See [Creating webhooks - GitHub Docs](https://docs.github.com/en/webhooks/using-webhooks/creating-webhooks) for creating a webhook and setting the secret token.
- `github_watch_push_branch`: The Git branch name to watch.
- `discord_webhook_url`: The Discord webhook URL to send the notification to Discord channel. See [Intro to Webhooks – Discord](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) for creating a Discord webhook URL.

Configure the Github Webhook. Set the payload URL to `<your-server-address>/webhooks/github`.

Copy the example `build.sh` to the current directory. Open it and add whatever commands you want to execute.

```txt
cp example/build.sh .
```

Finally, start the web server:

```txt
hooked server
# OR
hooked s
```

## License

MIT
