use clap::Parser;

use std::process::Command as ProcessCommand;

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: String,
    repository: String,
}

impl CreateCommand {
    pub fn handle(&self) {
        let user_config = grass::config::load_user_config().unwrap_or_default();
        let category = match grass::get_repository(&user_config, &self.category, &self.repository) {
            Some(category) => category,
            None => {
                eprintln!("Repository not found");
                return;
            }
        };

        let mut child = if let Ok(child) = ProcessCommand::new("tmux")
            .args(["new-session", "-d", "-s", &category.to_session_string()])
            .spawn()
        {
            child
        } else {
            eprintln!("Issue starting session");
            return;
        };

        match child.wait() {
            Ok(status) => {
                if let Some(0) = status.code() {
                    eprintln!("Opened tmux session {}", &category.to_session_string());
                } else {
                    eprintln!("Issue starting session");
                };
            }
            _ => eprintln!("Issue starting session"),
        }
    }
}
