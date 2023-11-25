use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use grass::dev::{strategy::api::SupportsAll, Api, RepositoryLocation};

use std::process::{Child, Command as ProcessCommand};

use crate::{
    error::CliError,
    facades::dialoguer::{select_category_and_repository, select_selectable},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Multiplexer {
    #[default]
    Tmux,
    Zellij,
}

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: Option<String>,
    repository: Option<String>,
    #[clap(short)]
    target: Option<Multiplexer>,
}

fn session_command_tmux(name: &str) -> Result<Child, CliError> {
    ProcessCommand::new("tmux")
        .args(["new-session", "-d", "-s", name])
        .spawn()
        .map_err(|_| CliError::new("Could not start tmux session!"))
}

fn session_command_zellij(_name: &str) -> Result<Child, CliError> {
    Err(CliError::not_supported("Currently it is not possible to create sessions in Zellij, workaround may come at some point"))
}

impl CreateCommand {
    fn create_session<T, U, V>(
        api: &Api<T>,
        category: U,
        repository: V,
        target: Multiplexer,
    ) -> Result<()>
    where
        T: SupportsAll,
        U: AsRef<str>,
        V: AsRef<str>,
    {
        let repository_location = grass::dev::resolve_repository_alias(
            api,
            RepositoryLocation::from((category.as_ref(), repository.as_ref())),
        )?;

        let mut child = match target {
            Multiplexer::Tmux => {
                session_command_tmux(repository_location.to_session_string().as_str())
            }
            Multiplexer::Zellij => {
                session_command_zellij(repository_location.to_session_string().as_str())
            }
        }?;

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

    fn select_repository<T: SupportsAll>(
        api: &Api<T>,
        category: &String,
        target: Multiplexer,
    ) -> Result<()> {
        let repositories: Vec<_> = grass::dev::list_repositories_in_category(api, category)?;
        let repositories: Vec<_> = repositories
            .into_iter()
            .map(|repository| repository.repository)
            .collect();

        let repository = select_selectable(&repositories).unwrap();

        Self::create_session(api, category, repository, target)?;
        Ok(())
    }

    fn select_category<T: SupportsAll>(api: &Api<T>, target: Multiplexer) -> Result<()> {
        let categories: Vec<_> = grass::dev::list_all_repositories(api)?;

        let repository = select_category_and_repository(categories.as_slice())
            .context("When running the command 'grass session create'")?;
        Self::create_session(api, &repository.category, &repository.repository, target)?;
        Ok(())
    }

    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        match self {
            CreateCommand {
                category: Some(category),
                repository: Some(repository),
                target,
            } => Self::create_session(
                api,
                category,
                repository,
                target.clone().unwrap_or_default(),
            )?,
            CreateCommand {
                category: Some(category),
                repository: None,
                target,
            } => Self::select_repository(api, category, target.clone().unwrap_or_default())?,
            CreateCommand { target, .. } => {
                Self::select_category(api, target.clone().unwrap_or_default())?
            }
        };
        Ok(())
    }
}
