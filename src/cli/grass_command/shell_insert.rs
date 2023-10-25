use std::{env, process::Command as ProcessCommand, str};

use anyhow::Result;
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{shells::Bash, Generator};
use grass::dev::{Api, strategy::api::SupportsAll};

use crate::error::CliError;

use super::GrassCommand;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shells {
    Bash,
}

#[derive(Parser, Debug)]
pub struct ShellInsertCommand {
    shell: Shells,
}

impl ShellInsertCommand {
    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        match self.shell {
            Shells::Bash => Self::handle_bash(api)?,
        };

        Ok(())
    }

    fn print_shell_complete<T: Generator>(generator: T) {
        let mut app = GrassCommand::command();
        let mut buf = Vec::new();
        clap_complete::generate(generator, &mut app, "grass", &mut buf);
        println!("{}", String::from_utf8(buf).unwrap_or_default());
    }

    fn handle_bash<T: SupportsAll>(api: &Api<T>) -> Result<()> {
        Self::print_shell_complete(Bash);

        println!(r#"gr() {{ cd "$(grass script path $@)"; }}"#);
        // Check for TMUX variable
        if env::var("TMUX").is_ok() {
            if let Ok(Ok(output)) = ProcessCommand::new("tmux")
                .args(["display-message", "-p", "#S"])
                .output()
                .map(|output| str::from_utf8(&output.stdout).map(String::from))
            {
                if let [repository, category] =
                    output.trim().split('@').collect::<Vec<_>>().as_slice()
                {
                    let path = grass::dev::get_repository_path_next(api, (*category, *repository))?;
                    let path = path.to_str().ok_or(CliError::new("Could not convert repository path to str"))?;
                    println!("cd {}", path);
                };
            };
        };
        Ok(())
    }
}
