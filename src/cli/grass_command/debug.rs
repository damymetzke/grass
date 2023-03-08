use clap::{Parser, Subcommand};
use grass::config;

#[derive(Debug, Subcommand)]
enum DebugSubcommand {
    Config,
}

#[derive(Parser, Debug)]
/// This command is used exclusively when debugging.
///
/// If you're seeing this it means GRAss is compiled in debug mode.
/// If you're trying to use this as an app, it means it has not been compiled correctly.
/// If you compiled it yourself make sure to compile it as a release.
/// If you got the program through some distribution please notify the distributer.
/// If you're debugging read the code to see what it does, I won't be documenting this any further.
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
