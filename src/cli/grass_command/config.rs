use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{strategy::api::SupportsAll, Api};

mod list;
mod explain;

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigSubcommand {
    List(list::ListCommand),
    Explain(explain::ExplainCommand),
}

#[derive(Parser, Debug, Clone)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

impl ConfigCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        match &self.command {
            ConfigSubcommand::List(command) => command.handle(api),
            ConfigSubcommand::Explain(command) => command.handle(api),
        }
    }
}
