use clap::Parser;
use grass::{
    config,
    repository::RepositoryChangeStatus,
    types::{FilteredCategoryDescription, SimpleRepositoryDescription},
};

use crate::output::generate_fancy_vertical_list;
use colored::*;

#[derive(Parser, Debug)]
pub struct ChangesCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
}

impl ChangesCommand {
    fn handle_category<T>(category: FilteredCategoryDescription<T, RepositoryChangeStatus>)
    where
        T: Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)>,
    {
        println!(
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
        )
    }

    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();

        match self {
            Self {
                category: Some(category),
                all: false,
            } => {
                let category = grass::list_repositories_with_change_status(&user_config, category);
                let category = if let Some(repositories) = category {
                    repositories
                        .filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate))
                } else {
                    eprintln!("Category or alias not found");
                    return;
                };
                Self::handle_category(category);
            }
            Self {
                category: None,
                all: true,
            } => {
                for category in grass::list_all_repositories_with_change_status(&user_config) {
                    let category = category
                        .filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate));
                    Self::handle_category(category);
                }
            }
            Self {
                category: Some(_),
                all: true,
            } => eprintln!("Received incompatible category with all flag"),
            Self {
                category: None,
                all: false,
            } => eprintln!("Provide either a category, or the --all flag"),
        }
    }
}
