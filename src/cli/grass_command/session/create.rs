use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::dev::config::{self, RootConfig};

use std::process::Command as ProcessCommand;

use crate::facades::dialoguer::select_repository;

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: Option<String>,
    repository: Option<String>,
}

impl CreateCommand {
    fn create_session<T, U>(user_config: &RootConfig, category: T, repository: U)
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let category = match grass::dev::get_repository(user_config, category, repository) {
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
        let category = grass::dev::list_repos_by_category(user_config, category).unwrap();

        let repository = select_repository(&category.repositories).unwrap();

        Self::create_session(user_config, &category.category, &repository.repository);
    }

    fn select_category(user_config: &RootConfig) {
        // TODO: Handle errors in this entire function
        let categories: Vec<String> = grass::dev::list_all_repositories(user_config)
            .iter()
            .flat_map(|category| {
                category
                    .repositories
                    .iter()
                    .map(|repository| format!("{}/{}", repository.category, repository.repository))
            })
            .collect();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&categories)
            .default(0)
            .vim_mode(true)
            .interact()
            .unwrap();

        let parts: Vec<&str> = categories[selection].splitn(2, '/').collect();

        let (category, repository) = match parts.as_slice() {
            [category, repository, ..] => (category, repository),
            _ => return,
        };
        Self::create_session(user_config, category, repository);
    }

    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();
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
