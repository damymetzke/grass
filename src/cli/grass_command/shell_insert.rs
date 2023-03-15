use std::{env, process::Command as ProcessCommand, str};

use clap::{Parser, ValueEnum};
use grass::config;

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

    fn handle_bash() {
        let user_config = config::load_user_config().unwrap_or_default();
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
                        grass::get_repository_path(&user_config, category, repository)
                            .map(|path| path.to_str().map(String::from))
                    {
                        println!("cd {}", path);
                    };
                };
            };
        };
    }
}
