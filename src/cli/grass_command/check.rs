mod changes;

use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{Api, strategy::api::SupportsAll};

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
    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        match &self.command {
            CheckSubCommand::Changes(changes_command) => changes_command.handle(api)?,
        };

        Ok(())
    }
}
