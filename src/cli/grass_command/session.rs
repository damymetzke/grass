pub mod create;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SessionSubcommand {
    Create(create::CreateCommand),
}

#[derive(Parser, Debug)]
pub struct SessionCommand {
    #[command(subcommand)]
    command: SessionSubcommand,
}

impl SessionCommand {
    pub fn handle(&self) {
        match &self.command {
            SessionSubcommand::Create(command) => command.handle(),
        }
    }
}
