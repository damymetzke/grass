use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::config::RootConfig;

use std::process::Command as ProcessCommand;

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: Option<String>,
    repository: Option<String>,
}

impl CreateCommand {
    fn create_session(user_config: &RootConfig, category: &String, repository: &String) {
        let category = match grass::get_repository(user_config, category, repository) {
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
        };
    }

    fn select_repository(user_config: &RootConfig, category: &String) {
        // TODO: Handle errors in this entire function
        let category = grass::list_repos_by_category(user_config, category).unwrap();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&category.repositories)
            .default(0)
            .interact()
            .unwrap();

        Self::create_session(
            user_config,
            &category.category,
            &category.repositories[selection],
        );
    }

    fn select_category(user_config: &RootConfig) {
        // TODO: Handle errors in this entire function
        let categories = grass::list_categories(user_config);

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&categories)
            .default(0)
            .interact()
            .unwrap();

        Self::select_repository(user_config, &categories[selection]);
    }

    pub fn handle(&self) {
        let user_config = grass::config::load_user_config().unwrap_or_default();
        match self {
            CreateCommand {
                category: Some(category),
                repository: Some(repository),
            } => Self::create_session(&user_config, category, repository),
            CreateCommand {
                category: Some(category),
                repository: None,
            } => Self::select_repository(&user_config, category),
            _ => Self::select_category(&user_config),
        }
    }
}
