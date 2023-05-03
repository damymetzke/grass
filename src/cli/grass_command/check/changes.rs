use clap::Parser;
use grass::config;
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

        println!(
            "┌ Repositories with uncommitted changes for '{}':\n│\n{}",
            category.category,
            category
                .repositories
                .iter()
                .sandwich_map(
                    |repository| format!("├─ {}", repository.repository),
                    |repository| format!("├─ {}", repository.repository),
                    |repository| format!("└─ {}", repository.repository),
                )
                .join("\n")
        );
    }
}
