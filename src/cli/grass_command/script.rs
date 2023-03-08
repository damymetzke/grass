mod path;

use clap::{Parser, Subcommand};

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
    pub fn handle(&self) {
        match &self.command {
            ScriptSubcommand::Path(path_command) => path_command.handle(),
        }
    }
}
