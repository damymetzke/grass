mod path;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum ScriptSubcommand {
    Path(path::PathCommand),
}

#[derive(Parser, Debug)]
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
