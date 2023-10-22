use clap::{Command};

pub fn parse_args() -> Command {
    Command::new("hooked")
        .about("A command-line CI/CD tool that triggers custom \
                build and deployment scripts when it receives \
                GitHub webhook messages.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Create a new config file")
        )
        .subcommand(
            Command::new("server")
                .about("Start a server")
                .alias("s")
        )
}
