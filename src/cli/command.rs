mod ls;
mod script;
use clap::Subcommand;

#[cfg(debug_assertions)]
use crate::debug;

#[derive(Debug, Subcommand)]
pub enum Command {
    Ls(ls::LsCommand),
    Script(script::ScriptCommand),
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
}

impl Command {
    pub fn handle(&self) {
        match self {
            Command::Ls(command) => command.handle(),
            Command::Script(command) => command.handle(),
            #[cfg(debug_assertions)]
            Command::Debug(command) => debug::handle_debug(command),
        }
    }
}
