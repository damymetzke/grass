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

        let repositories =
            grass::list_repositories_with_change_status(&user_config, &self.category);
        let (repositories, category_name) = if let Some(repositories) = repositories {
            (
                repositories
                    .repository_iterator
                    .filter(|(_, status)| !matches!(status, RepositoryChangeStatus::UpToDate)),
                repositories.category,
            )
        } else {
            eprintln!("Category or alias not found");
            return;
        };

        println!(
            "┌ Repositories with uncommitted changes for '{}':\n│\n{}",
            category_name,
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
