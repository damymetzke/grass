mod clean;
mod clone;

use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{strategy::api::ApiStrategy, Api};

#[derive(Debug, Subcommand)]
pub enum RepoSubcommand {
    Clean(clean::CleanCommand),
    Clone(clone::CloneCommand),
}

#[derive(Parser, Debug)]
pub struct RepoCommand {
    #[command(subcommand)]
    command: RepoSubcommand,
}

impl RepoCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: ApiStrategy,
    {
        match &self.command {
            RepoSubcommand::Clean(command) => command.handle(api),
            RepoSubcommand::Clone(command) => command.handle(),
        }
    }
}
