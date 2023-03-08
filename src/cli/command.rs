mod ls;
use clap::Subcommand;

#[cfg(debug_assertions)]
use crate::debug;
use crate::script_command::{self, ScriptCommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Ls(ls::LsCommand),
    Script(ScriptCommand),
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
}

impl Command {
    pub fn handle(&self) {
        match self {
            Command::Ls(command) => command.handle(),
            Command::Script(command) => script_command::handle_script(command),
            #[cfg(debug_assertions)]
            Command::Debug(command) => debug::handle_debug(command),
        }
    }
}
