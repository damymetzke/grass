mod clean;
mod clone;

use clap::{Parser, Subcommand};

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
    pub fn handle(&self) {
        match &self.command {
            RepoSubcommand::Clean(command) => command.handle(),
            RepoSubcommand::Clone(command) => {command.handle().ok();},
        }
    }
}
