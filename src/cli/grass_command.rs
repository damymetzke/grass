mod check;
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
use grass::dev::{strategy::api::ApiStrategy, Api};

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
    #[cfg(debug_assertions)]
    Debug(debug::DebugCommand),
    #[clap(external_subcommand)]
    External(Vec<String>),
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

enum HandleExternalResult {
    CommandFound,
    CommandNotFound(String),
}

impl GrassCommand {
    fn handle_external(
        value: &Vec<String>,
        commands: &Vec<ExternalCommand>,
    ) -> HandleExternalResult {
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

    pub fn handle<T>(&self, api: &Api<T>, external_commands: &Vec<ExternalCommand>) -> Result<()>
    where
        T: ApiStrategy,
    {
        match &self.command {
            GrassSubcommand::Check(command) => command.handle(),
            GrassSubcommand::Ls(command) => command.handle(),
            GrassSubcommand::Repo(command) => command.handle(api)?,
            GrassSubcommand::Script(command) => command.handle(),
            GrassSubcommand::ShellInsert(command) => command.handle(),
            GrassSubcommand::Session(command) => command.handle(),
            GrassSubcommand::Cs(command) => command.handle(),
            #[cfg(debug_assertions)]
            GrassSubcommand::Debug(command) => command.handle(),
            GrassSubcommand::External(parts) => {
                let result = Self::handle_external(parts, external_commands);
                match result {
                    HandleExternalResult::CommandNotFound(cmd) => {
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
                    _ => (),
                }
            }
        };
        Ok(())
    }
}
