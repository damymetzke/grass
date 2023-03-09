#[cfg(debug_assertions)]
mod debug;
mod ls;
mod script;
mod shell_insert;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum GrassSubcommand {
    Ls(ls::LsCommand),
    Script(script::ScriptCommand),
    ShellInsert(shell_insert::ShellInsertCommand),
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
/// GRAss: GLobal Repository Assistent
///
/// GRAss is a utility which aims to assit you into managing repositories.
/// For more information see:
/// <https://github.com/damymetzke/grass>
pub struct GrassCommand {
    #[command(subcommand)]
    command: GrassSubcommand,
}

impl GrassCommand {
    pub fn handle(&self) {
        match &self.command {
            GrassSubcommand::Ls(command) => command.handle(),
            GrassSubcommand::Script(command) => command.handle(),
            GrassSubcommand::ShellInsert(command) => command.handle(),
            #[cfg(debug_assertions)]
            GrassSubcommand::Debug(command) => command.handle(),
        }
    }
}
