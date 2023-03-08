#[cfg(debug_assertions)]
mod debug;
mod ls;
mod script;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum GrassSubcommand {
    Ls(ls::LsCommand),
    Script(script::ScriptCommand),
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GrassCommand {
    #[command(subcommand)]
    command: GrassSubcommand,
}

impl GrassCommand {
    pub fn handle(&self) {
        match &self.command {
            GrassSubcommand::Ls(command) => command.handle(),
            GrassSubcommand::Script(command) => command.handle(),
            #[cfg(debug_assertions)]
            GrassSubcommand::Debug(command) => command.handle(),
        }
    }
}
