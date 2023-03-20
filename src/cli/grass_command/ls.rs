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
    /// Only show repositories with uncommitted changes
    #[clap(long)]
    uncommitted: bool,
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

    fn generate_output_all_repositories_by_category<T, U>(categories: T) -> String
    where
        T: IntoIterator<Item = U>,
        U: std::borrow::Borrow<SimpleCategoryDescription>,
    {
        categories
            .into_iter()
            .map(|category| Self::generate_output_repositories_for_category(category.borrow()))
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
                uncommitted: false,
            } => Self::generate_output_category_name_only(
                grass::list_categories(&user_config).iter(),
            ),
            LsCommand {
                category: Some(category),
                all: false,
                uncommitted: false,
            } => Self::generate_output_repositories_for_category(
                &grass::list_repos_by_category(&user_config, category).unwrap(),
            ),
            LsCommand {
                category: None,
                all: true,
                uncommitted: false,
            } => Self::generate_output_all_repositories_by_category(grass::list_all_repositories(
                &user_config,
            )),
            LsCommand {
                category: Some(category),
                all: false,
                uncommitted: true,
            } => Self::generate_output_repositories_for_category(
                &grass::list_repos_by_category(&user_config, category)
                    .unwrap()
                    .uncommitted_changes_only(),
            ),
            LsCommand {
                category: None,
                all: true,
                uncommitted: true,
            } => Self::generate_output_all_repositories_by_category(
                grass::list_all_repositories(&user_config)
                    .into_iter()
                    .map(|repository| repository.uncommitted_changes_only()),
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
