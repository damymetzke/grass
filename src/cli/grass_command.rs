mod check;
mod config;
#[cfg(debug_assertions)]
mod debug;
mod ls;
mod repo;
mod script;
mod session;
mod shell_insert;

use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use grass::dev::{strategy::api::SupportsAll, Api};

use crate::external_command::ExternalCommand;

#[derive(Debug, Subcommand)]
pub enum GrassSubcommand {
    Check(check::CheckCommand),
    Ls(ls::LsCommand),
    Repo(repo::RepoCommand),
    Script(script::ScriptCommand),
    ShellInsert(shell_insert::ShellInsertCommand),
    Session(session::SessionCommand),
    Cs(session::create::CreateCommand),
    Config(config::ConfigCommand),
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
    #[clap(external_subcommand)]
    External(Vec<String>),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
/// GRAss: GLobal Repository Assistent
///
/// A utility which aims to assist you into managing repositories.
/// The repository can be found at <https://github.com/damymetzke/grass>.
///
/// You can add aliases by puttin scripts with the prefix 'grass-' on your path.
/// For example, the script 'grass-foo'.
/// This will allow you to call this script using `grass foo`.
/// Additional arguments will be passed to the script.
pub struct GrassCommand {
    #[command(subcommand)]
    command: GrassSubcommand,
    #[arg(short='v', action=clap::ArgAction::Count)]
    verbose: u8,
}

enum HandleExternalResult {
    CommandFound,
    CommandNotFound(String),
}

pub enum Verbosity {
    Warn,
    Info,
    Debug,
    Trace,
}

impl GrassCommand {
    fn handle_external(value: &Vec<String>, commands: &[ExternalCommand]) -> HandleExternalResult {
        let (subcommand, args) = match value.as_slice() {
            [subcommand, args @ ..] => (subcommand, args),
            _ => return HandleExternalResult::CommandNotFound(String::new()),
        };

        let command = commands
            .iter()
            .find(|command| command.command == subcommand.as_str());

        let command = match command {
            Some(command) => command,
            None => return HandleExternalResult::CommandNotFound(subcommand.clone()),
        };

        Command::new(&command.path).args(args).status().ok();

        HandleExternalResult::CommandFound
    }

    pub fn handle<T>(&self, api: &Api<T>, external_commands: &[ExternalCommand]) -> Result<()>
    where
        T: SupportsAll,
    {
        match &self.command {
            GrassSubcommand::Check(command) => command.handle(api)?,
            GrassSubcommand::Ls(command) => command.handle(api)?,
            GrassSubcommand::Repo(command) => command.handle(api)?,
            GrassSubcommand::Script(command) => command.handle(api)?,
            GrassSubcommand::ShellInsert(command) => command.handle(),
            GrassSubcommand::Session(command) => command.handle(api)?,
            GrassSubcommand::Cs(command) => command.handle(api)?,
            GrassSubcommand::Config(command) => command.handle(api)?,
            #[cfg(debug_assertions)]
            GrassSubcommand::Debug(command) => command.handle(),
            GrassSubcommand::External(parts) => {
                let result = Self::handle_external(parts, external_commands);
                if let HandleExternalResult::CommandNotFound(cmd) = result {
                    eprintln!(
                        "{} unrecognized subcommand '{}'",
                        "error:".red().bold(),
                        cmd.yellow()
                    );
                    eprintln!();
                    eprintln!(
                        "{} {} <COMMAND>",
                        "Usage:".bold().underline(),
                        "grass".bold()
                    );
                    eprintln!();
                    eprintln!("For more information, try '{}'.", "--help".bold());
                }
            }
        };
        Ok(())
    }

    pub fn get_verbosity(&self) -> Verbosity {
        match self.verbose {
            0 => Verbosity::Warn,
            1 => Verbosity::Info,
            2 => Verbosity::Debug,
            _ => Verbosity::Trace,
        }
    }
}
