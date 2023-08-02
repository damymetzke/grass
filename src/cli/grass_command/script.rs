mod path;

use anyhow::Result;
use clap::{Parser, Subcommand};
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Debug, Subcommand)]
enum ScriptSubcommand {
    Path(path::PathCommand),
}

#[derive(Parser, Debug)]
/// Parent command for all actions primarily intended for scripting
///
/// All sub commands are intended for usage by scripts, instead of the end user.
/// As such all commands will be delivered in a format optimized for scripting.
/// This means no trailing newlines, no colors, and not always human readable.
pub struct ScriptCommand {
    #[command(subcommand)]
    command: ScriptSubcommand,
}

impl ScriptCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        match &self.command {
            ScriptSubcommand::Path(path_command) => path_command.handle(api),
        }
    }
}
