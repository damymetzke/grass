use clap::Parser;
use grass::{config, types::SimpleCategoryDescription};
use itertools::Itertools;

use crate::output::generate_fancy_vertical_list;

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
        generate_fancy_vertical_list(
            format!("Repos for category '{}'", category.category),
            category
                .repositories
                .iter()
                .map(|repository| &repository.repository),
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
        generate_fancy_vertical_list("Categories", categories)
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
            } => Self::generate_output_all_repositories_by_category(grass::list_all_repositories(
                &user_config,
            )),
            _ => {
                // TODO: Generate more specific output
                eprintln!("There was a problem with the command");
                return;
            }
        };

        println!("{}", output);
    }
}
