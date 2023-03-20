use clap::Parser;
use grass::{config, types::SimpleCategoryDescription};
use itertools::Itertools;

use crate::more_itertools::MoreItertools;

#[derive(Parser, Debug)]
/// List categories and repositories
///
/// Invoke with no commands to list the available categories.
/// Categories are defined in the configuration file.
pub struct LsCommand {
    /// The category to list, can be an alias
    ///
    /// When given will list the repositories in the category.
    category: Option<String>,
    /// List all repositories in all categories
    #[clap(short, long)]
    all: bool,
}

impl LsCommand {
    fn generate_output_repositories_for_category(category: &SimpleCategoryDescription) -> String {
        format!(
            "┌ Repos for category '{}':\n│\n{}",
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
        )
    }

    fn generate_output_all_repositories_by_category<'a, T>(categories: T) -> String
    where
        T: IntoIterator<Item = &'a SimpleCategoryDescription>,
    {
        categories
            .into_iter()
            .map(Self::generate_output_repositories_for_category)
            .join("\n\n")
    }

    fn generate_output_category_name_only<'a, T>(categories: T) -> String
    where
        T: IntoIterator<Item = &'a String>,
    {
        format!(
            "┌ Categories:\n│\n{}",
            categories
                .into_iter()
                .sandwich_map(
                    |category| format!("├─ {}", category),
                    |category| format!("├─ {}", category),
                    |category| format!("└─ {}", category),
                )
                .join("\n")
        )
    }

    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();

        // TODO: Handle errors
        let output = match self {
            LsCommand {
                category: None,
                all: false,
            } => Self::generate_output_category_name_only(
                grass::list_categories(&user_config).iter(),
            ),
            LsCommand {
                category: Some(category),
                all: false,
            } => Self::generate_output_repositories_for_category(
                &grass::list_repos_by_category(&user_config, category).unwrap(),
            ),
            LsCommand {
                category: None,
                all: true,
            } => Self::generate_output_all_repositories_by_category(
                grass::list_all_repositories(&user_config).iter(),
            ),
            _ => {
                // TODO: Generate more specific output
                eprintln!("There was a problem with the command");
                return;
            }
        };

        println!("{}", output);
    }
}
