use clap::{Parser, Subcommand};
use grass::config;

#[derive(Debug, Subcommand)]
enum DebugSubcommand {
    Config,
}

#[derive(Parser, Debug)]
pub struct DebugCommand {
    #[command(subcommand)]
    command: DebugSubcommand,
}

impl DebugCommand {
    pub fn handle(&self) {
        match &self.command {
            DebugSubcommand::Config => println!("{:?}", config::load_user_config().unwrap()),
        }
    }
}
