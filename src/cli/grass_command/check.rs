mod changes;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum CheckSubCommand {
    Changes(changes::ChangesCommand),
}

#[derive(Parser, Debug)]
pub struct CheckCommand {
    #[command(subcommand)]
    command: CheckSubCommand,
}

impl CheckCommand {
    pub fn handle(&self) {
        match &self.command {
            CheckSubCommand::Changes(changes_command) => changes_command.handle(),
        }
    }
}
