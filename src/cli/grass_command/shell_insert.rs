use std::{env, process::Command as ProcessCommand, str};

use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{shells::Bash, Generator};
use grass::dev::config;

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
    pub fn handle(&self) {
        match self.shell {
            Shells::Bash => Self::handle_bash(),
        };
    }

    fn print_shell_complete<T: Generator>(generator: T) {
        let mut app = GrassCommand::command();
        let mut buf = Vec::new();
        clap_complete::generate(generator, &mut app, "grass", &mut buf);
        println!("{}", String::from_utf8(buf).unwrap_or_default());
    }

    fn handle_bash() {
        Self::print_shell_complete(Bash);

        let user_config = config::load_user_config().unwrap();
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
                    if let Some(Some(path)) =
                        grass::dev::get_repository_path(&user_config.grass, category, repository)
                            .map(|path| path.to_str().map(String::from))
                    {
                        println!("cd {}", path);
                    };
                };
            };
        };
    }
}
