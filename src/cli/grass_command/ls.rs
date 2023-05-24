use clap::{Parser, ValueEnum};
use grass::{config, types::SimpleCategoryDescription};
use itertools::Itertools;

use crate::output::generate_fancy_vertical_list;

#[derive(ValueEnum, Debug, Clone, Default)]
enum Format {
    #[default]
    Fancy,
    Simple,
}

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
    #[clap(long)]
    format: Option<Format>,
}

impl LsCommand {
    fn generate_output_repositories_for_category(
        category: &SimpleCategoryDescription,
        format: &Format,
    ) -> String {
        match format {
            Format::Fancy => generate_fancy_vertical_list(
                format!("Repos for category '{}'", category.category),
                category
                    .repositories
                    .iter()
                    .map(|repository| &repository.repository),
            ),
            Format::Simple => category
                .repositories
                .iter()
                .map(|repository| format!("{}/{}", &repository.category, &repository.repository))
                .join(" "),
        }
    }

    fn generate_output_all_repositories_by_category<T, U>(categories: T, format: &Format) -> String
    where
        T: IntoIterator<Item = U>,
        U: std::borrow::Borrow<SimpleCategoryDescription>,
    {
        categories
            .into_iter()
            .map(|category| {
                Self::generate_output_repositories_for_category(category.borrow(), format)
            })
            .join(match format {
                Format::Fancy => "\n\n",
                Format::Simple => " ",
            })
    }

    fn generate_output_category_name_only<'a, T>(categories: T, format: &Format) -> String
    where
        T: IntoIterator<Item = &'a String>,
    {
        match format {
            Format::Fancy => generate_fancy_vertical_list("Categories", categories),
            Format::Simple => categories.into_iter().join(" "),
        }
    }

    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();

        // TODO: Handle errors
        let output = match self {
            LsCommand {
                category: None,
                all: false,
                format,
            } => Self::generate_output_category_name_only(
                grass::list_categories(&user_config).iter(),
                &format.clone().unwrap_or_default(),
            ),
            LsCommand {
                category: Some(category),
                all: false,
                format,
            } => Self::generate_output_repositories_for_category(
                &grass::list_repos_by_category(&user_config, category).unwrap(),
                &format.clone().unwrap_or_default(),
            ),
            LsCommand {
                category: None,
                all: true,
                format,
            } => Self::generate_output_all_repositories_by_category(
                grass::list_all_repositories(&user_config),
                &format.clone().unwrap_or_default(),
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
