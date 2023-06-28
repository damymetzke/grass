pub mod create;

use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{strategy::api::ApiStrategy, Api};

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
    pub fn handle<T: ApiStrategy>(&self, api: &Api<T>) -> Result<()> {
        match &self.command {
            SessionSubcommand::Create(command) => command.handle(api)?,
        };
        Ok(())
    }
}
