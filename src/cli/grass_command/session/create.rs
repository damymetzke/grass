use anyhow::{Context, Result};
use clap::Parser;
use grass::dev::{
    config::{self, RootConfig},
    strategy::api::SupportsAll,
    Api,
};

use std::process::Command as ProcessCommand;

use crate::facades::dialoguer::{select_category_and_repository, select_selectable};

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

    fn select_repository<T: SupportsAll>(
        api: &Api<T>,
        user_config: &RootConfig,
        category: &String,
    ) -> Result<()> {
        let repositories: Vec<_> = grass::dev::list_repositories_in_category(api, category)?;
        let repositories: Vec<_> = repositories
            .into_iter()
            .map(|repository| repository.repository)
            .collect();

        let repository = select_selectable(&repositories).unwrap();

        Self::create_session(user_config, category, repository);
        Ok(())
    }

    fn select_category<T: SupportsAll>(user_config: &RootConfig, api: &Api<T>) -> Result<()> {
        let categories: Vec<_> = grass::dev::list_all_repositories(api)?;

        let repository = select_category_and_repository(categories.as_slice())
            .context("When running the command 'grass session create'")?;
        Self::create_session(user_config, &repository.category, &repository.repository);
        Ok(())
    }

    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        let user_config = config::load_user_config().unwrap();
        match self {
            CreateCommand {
                category: Some(category),
                repository: Some(repository),
            } => Self::create_session(&user_config, category, repository),
            CreateCommand {
                category: Some(category),
                repository: None,
            } => Self::select_repository(api, &user_config, category)?,
            _ => Self::select_category(&user_config, api)?,
        };
        Ok(())
    }
}
