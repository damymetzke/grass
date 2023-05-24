use clap::{Parser, ValueEnum};
use grass::{
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
    ) where
        T: Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)>,
    {
        match format {
            Format::Fancy => println!(
                "{}",
                generate_fancy_vertical_list(
                    format!(
                        "Repositories with uncommitted changes for '{}'",
                        category.category
                    ),
                    category
                        .repository_iterator
                        .map(|(repository, status)| format!(
                            "{} ({})",
                            repository.repository.bright_blue(),
                            status.to_string().red()
                        ))
                )
            ),
            Format::Simple => print!(
                "{}",
                category
                    .repository_iterator
                    .map(|(repository, status)| match status {
                        RepositoryChangeStatus::UpToDate =>
                            panic!("No up-to-date should exist at this point"),
                        RepositoryChangeStatus::NoRepository => format!(
                            "{}/{} no_repository 0",
                            repository.category, repository.repository
                        ),
                        RepositoryChangeStatus::UncommittedChanges(n) => format!(
                            "{}/{} uncommitted_changes {}",
                            repository.category, repository.repository, n
                        ),
                    })
                    .join("\n")
            ),
        }
    }

    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();

        match self {
            Self {
                category: Some(category),
                all: false,
                format,
            } => {
                let category = grass::list_repositories_with_change_status(&user_config, category);
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
                let mut insert_newline = false;
                for category in grass::list_all_repositories_with_change_status(&user_config) {
                    if insert_newline {
                        println!();
                    };
                    insert_newline = true;

                    let category = category
                        .filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate));
                    Self::handle_category(category, &format.clone().unwrap_or_default());
                }
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
