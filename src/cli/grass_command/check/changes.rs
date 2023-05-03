use clap::Parser;
use grass::{config, repository::RepositoryChangeStatus};
use itertools::Itertools;

use crate::more_itertools::MoreItertools;

#[derive(Parser, Debug)]
pub struct ChangesCommand {
    category: String,
}

impl ChangesCommand {
    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();

        let category = grass::list_repos_by_category(&user_config, &self.category);
        let category = if let Some(category) = category {
            category
        } else {
            eprintln!("Category or alias not found");
            return;
        };

        let repositories = grass::list_repositories_with_change_status(&user_config, &self.category);
        let repositories = if let Some(repositories) = repositories {
            repositories.filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate))
        } else {
            eprintln!("Category or alias not found");
            return;
        };

        println!(
            "┌ Repositories with uncommitted changes for '{}':\n│\n{}",
            category.category,
            repositories
                .sandwich_map(
                    |(repository, _)| format!("├─ {}", repository.repository),
                    |(repository, _)| format!("├─ {}", repository.repository),
                    |(repository, _)| format!("└─ {}", repository.repository),
                )
                .join("\n")
        );
    }
}
