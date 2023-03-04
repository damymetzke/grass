use clap::{Parser, Subcommand};
use grass::config;

#[derive(Debug, Subcommand)]
enum Command {
    Config,
}

#[derive(Parser, Debug)]
pub struct DebugCommand {
    #[command(subcommand)]
    command: Command,
}

pub fn handle_debug(command: DebugCommand) {
    match command.command {
        Command::Config => println!("{:?}", config::load_user_config().unwrap()),
    }
}
