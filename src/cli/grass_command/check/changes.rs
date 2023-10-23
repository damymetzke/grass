use anyhow::Result;
use clap::{Parser, ValueEnum};
use grass::dev::{
    iterator::{
        location::LocationIterExtensions,
        location_and_change_status::LocationAndChangeStatusIterExtensions,
    },
    list_all_repositories, list_repositories_in_category,
    strategy::{api::SupportsAll, git::RepositoryChangeStatus},
    Api, RepositoryLocation,
};
use itertools::Itertools;

use crate::{error::CliError, more_itertools::MoreItertools};

#[derive(ValueEnum, Debug, Clone, Default)]
enum Format {
    #[default]
    Fancy,
    Simple,
}

#[derive(Parser, Debug)]
pub struct ChangesCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
    #[clap(long)]
    format: Option<Format>,
}

impl ChangesCommand {
    fn display_plain(&self, repositories: Box<[(RepositoryLocation, RepositoryChangeStatus)]>) {
        for (location, change_status) in repositories.iter() {
            let change_status = match change_status {
                RepositoryChangeStatus::UpToDate => String::from("up_to_date"),
                RepositoryChangeStatus::NoRepository => String::from("no_repository"),
                RepositoryChangeStatus::UncommittedChanges { num_changes } => {
                    num_changes.to_string()
                }
                RepositoryChangeStatus::Unknown => String::from("unknown_status"),
            };

            println!(
                "{} {} {}",
                location.category, location.repository, change_status
            )
        }
    }

    fn display_pretty(&self, repositories: Box<[(RepositoryLocation, RepositoryChangeStatus)]>) {
        let result = (&repositories
            .into_iter()
            .group_by(|(location, _)| &location.category))
            .into_iter()
            .map(|(category, repositories)| {
                let repositories: Box<[_]> = repositories.sandwich_map(
                    |(RepositoryLocation { repository, .. }, change_status)| {
                        format!("\n├ {} {}", repository, change_status)
                    },
                    |(RepositoryLocation { repository, .. }, change_status)| {
                        format!("\n├ {} {}", repository, change_status)
                    },
                    |(RepositoryLocation { repository, .. }, change_status)| {
                        format!("\n└ {} {}", repository, change_status)
                    },
                ).collect();

                format!("┌── {}\n│{}\n", category, repositories.join(""))
            })
            .join("");

        eprintln!("{}", result)
    }

    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        let mut repositories: Box<[_]> = match self {
            Self {
                category: Some(category),
                all: false,
                ..
            } => {
                let repositories: Vec<_> = list_repositories_in_category(api, category)?;
                repositories
                    .into_iter()
                    .with_change_status(api)
                    .uncommitted_changes_only()
                    .collect()
            }
            Self {
                category: None,
                all: true,
                ..
            } => {
                let repositories: Vec<_> = list_all_repositories(api)?;
                repositories
                    .into_iter()
                    .with_change_status(api)
                    .uncommitted_changes_only()
                    .collect()
            }
            Self {
                category: Some(_),
                all: true,
                ..
            } => return Err(CliError::new("Received incompatible category with all flag").into()),
            Self {
                category: None,
                all: false,
                ..
            } => return Err(CliError::new("Provide either a category, or the --all flag").into()),
        };

        repositories.sort();

        match self.format {
            Some(Format::Simple) => self.display_plain(repositories),
            _ => self.display_pretty(repositories),
        };

        Ok(())
    }
}
