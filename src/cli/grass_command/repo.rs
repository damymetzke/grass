mod clean;
mod clone;
mod create;
mod rename;

use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Debug, Subcommand)]
pub enum RepoSubcommand {
    Clean(clean::CleanCommand),
    Clone(clone::CloneCommand),
    Create(create::CreateCommand),
    Rename(rename::RenameCommand),
}

#[derive(Parser, Debug)]
pub struct RepoCommand {
    #[command(subcommand)]
    command: RepoSubcommand,
}

impl RepoCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        match &self.command {
            RepoSubcommand::Clean(command) => command.handle(api),
            RepoSubcommand::Clone(command) => command.handle(api),
            RepoSubcommand::Create(command) => command.handle(api),
            RepoSubcommand::Rename(command) => command.handle(api),
        }
    }
}
