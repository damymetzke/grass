use std::io::{self, Write};

use clap::{Parser, ValueEnum};
use grass::dev::{
    config,
    repository::RepositoryChangeStatus,
    types::{FilteredCategoryDescription, SimpleRepositoryDescription},
};
use itertools::Itertools;

use crate::output::generate_fancy_vertical_list;
use colored::*;

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
    fn handle_category<T>(
        category: FilteredCategoryDescription<T, RepositoryChangeStatus>,
        format: &Format,
    ) -> String
    where
        T: Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)>,
    {
        match format {
            Format::Fancy => generate_fancy_vertical_list(
                format!(
                    "Repositories with uncommitted changes for '{}'",
                    category.category
                ),
                category.repository_iterator.map(|(repository, status)| {
                    format!(
                        "{} ({})",
                        repository.repository.bright_blue(),
                        status.to_string().red()
                    )
                }),
            ),
            Format::Simple => category
                .repository_iterator
                .map(|(repository, status)| match status {
                    RepositoryChangeStatus::UpToDate => {
                        panic!("No up-to-date should exist at this point")
                    }
                    RepositoryChangeStatus::NoRepository => format!(
                        "{}/{} no_repository 0",
                        repository.category, repository.repository
                    ),
                    RepositoryChangeStatus::UncommittedChanges(n) => format!(
                        "{}/{} uncommitted_changes {}",
                        repository.category, repository.repository, n
                    ),
                })
                .join("\n"),
        }
    }

    pub fn handle(&self) {
        // TODO: Handle error
        let user_config = config::load_user_config().unwrap();

        match self {
            Self {
                category: Some(category),
                all: false,
                format,
            } => {
                let category =
                    grass::dev::list_repositories_with_change_status(&user_config, category);
                let category = if let Some(repositories) = category {
                    repositories
                        .filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate))
                } else {
                    eprintln!("Category or alias not found");
                    return;
                };
                Self::handle_category(category, &format.clone().unwrap_or_default());
            }
            Self {
                category: None,
                all: true,
                format,
            } => {
                let result = grass::dev::list_all_repositories_with_change_status(&user_config)
                    .into_iter()
                    .map(|category| {
                        let category = category.filter(|(_, status)| {
                            !matches!(status, RepositoryChangeStatus::UpToDate)
                        });
                        Self::handle_category(category, &format.clone().unwrap_or_default())
                    })
                    .join(match format.clone().unwrap_or_default() {
                        Format::Fancy => "\n\n",
                        Format::Simple => "\n",
                    });
                print!("{}", result);
                io::stdout().flush().expect("Unable to flush");
            }
            Self {
                category: Some(_),
                all: true,
                format: _,
            } => eprintln!("Received incompatible category with all flag"),
            Self {
                category: None,
                all: false,
                format: _,
            } => eprintln!("Provide either a category, or the --all flag"),
        }
    }
}
