use clap::Parser;
use grass::{
    config,
    repository::RepositoryChangeStatus,
    types::{FilteredCategoryDescription, SimpleRepositoryDescription},
};
use itertools::Itertools;

use crate::more_itertools::MoreItertools;

#[derive(Parser, Debug)]
pub struct ChangesCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
}

impl ChangesCommand {
    fn handle_category<T, U>(category: FilteredCategoryDescription<T, U>)
    where
        T: Iterator<Item = (SimpleRepositoryDescription, U)>,
    {
        println!(
            "┌ Repositories with uncommitted changes for '{}':\n│\n{}",
            category.category,
            category
                .repository_iterator
                .sandwich_map(
                    |(repository, _)| format!("├─ {}", repository.repository),
                    |(repository, _)| format!("├─ {}", repository.repository),
                    |(repository, _)| format!("└─ {}", repository.repository),
                )
                .join("\n")
        );
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
            } => todo!(),
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
