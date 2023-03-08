use clap::Parser;
use grass::{config, list_categories, list_repos_by_category};
use itertools::Itertools;

use crate::more_itertools::MoreItertools;

#[derive(Parser, Debug)]
pub struct LsCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
}

impl LsCommand {
    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();
        let category = if let Some(result) = &self.category {
            result
        } else if self.all {
            let categories = grass::list_all_repositories(&user_config);

            println!(
                "{}",
                categories
                    .iter()
                    .map(|category| format!(
                        "┌ Repos for category '{}':\n│\n{}",
                        category.category,
                        category
                            .repositories
                            .iter()
                            .sandwich_map(
                                |category_name| format!("├─ {}", category_name),
                                |category_name| format!("├─ {}", category_name),
                                |category_name| format!("└─ {}", category_name),
                            )
                            .join("\n")
                    ))
                    .join("\n\n")
            );
            return;
        } else {
            let categories = list_categories(&user_config);
            println!(
                "┌ Categories:\n│\n{}",
                categories
                    .iter()
                    .sandwich_map(
                        |category_name| format!("├─ {}", category_name),
                        |category_name| format!("├─ {}", category_name),
                        |category_name| format!("└─ {}", category_name),
                    )
                    .join("\n")
            );
            return;
        };

        let (category, repositories) =
            if let Some(category) = list_repos_by_category(&user_config, category) {
                (category.category, category.repositories)
            } else {
                eprintln!("Category '{}' does not exist", &category);
                return;
            };
        println!(
            "┌ Repos for category '{}':\n│\n{}",
            category,
            repositories
                .iter()
                .sandwich_map(
                    |category| format!("├─ {}", category),
                    |category| format!("├─ {}", category),
                    |category| format!("└─ {}", category),
                )
                .join("\n")
        );
    }
}
