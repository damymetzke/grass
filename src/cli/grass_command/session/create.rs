use anyhow::{Context, Result};
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api, RepositoryLocation};

use std::process::Command as ProcessCommand;

use crate::{
    error::CliError,
    facades::dialoguer::{select_category_and_repository, select_selectable},
};

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: Option<String>,
    repository: Option<String>,
}

impl CreateCommand {
    fn create_session<T, U, V>(api: &Api<T>, category: U, repository: V) -> Result<()>
    where
        T: SupportsAll,
        U: AsRef<str>,
        V: AsRef<str>,
    {
        let repository_location = grass::dev::resolve_repository_alias(
            api,
            RepositoryLocation::from((category.as_ref(), repository.as_ref())),
        )?;

        let mut child = if let Ok(child) = ProcessCommand::new("tmux")
            .args([
                "new-session",
                "-d",
                "-s",
                &repository_location.to_session_string(),
            ])
            .spawn()
        {
            child
        } else {
            return Err(CliError::new("Could not start tmux session!").into());
        };

        match child.wait() {
            Ok(status) => {
                if let Some(0) = status.code() {
                    eprintln!(
                        "Opened tmux session {}",
                        &repository_location.to_session_string()
                    );
                } else {
                    eprintln!("Issue starting session");
                };
            }
            _ => eprintln!("Issue starting session"),
        };
        Ok(())
    }

    fn select_repository<T: SupportsAll>(api: &Api<T>, category: &String) -> Result<()> {
        let repositories: Vec<_> = grass::dev::list_repositories_in_category(api, category)?;
        let repositories: Vec<_> = repositories
            .into_iter()
            .map(|repository| repository.repository)
            .collect();

        let repository = select_selectable(&repositories).unwrap();

        Self::create_session(api, category, repository)?;
        Ok(())
    }

    fn select_category<T: SupportsAll>(api: &Api<T>) -> Result<()> {
        let categories: Vec<_> = grass::dev::list_all_repositories(api)?;

        let repository = select_category_and_repository(categories.as_slice())
            .context("When running the command 'grass session create'")?;
        Self::create_session(api, &repository.category, &repository.repository)?;
        Ok(())
    }

    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        match self {
            CreateCommand {
                category: Some(category),
                repository: Some(repository),
            } => Self::create_session(api, category, repository)?,
            CreateCommand {
                category: Some(category),
                repository: None,
            } => Self::select_repository(api, category)?,
            _ => Self::select_category(api)?,
        };
        Ok(())
    }
}
