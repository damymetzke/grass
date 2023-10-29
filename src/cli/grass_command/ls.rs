use anyhow::Result;
use clap::{Parser, ValueEnum};
use grass::dev::{strategy::api::SupportsAll, Api, RepositoryLocation};
use itertools::Itertools;

use crate::{
    cli_result::{CliOutput, CliResult},
    error::CliError,
    output::generate_fancy_vertical_list,
};

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
        category: String,
        repositories: Vec<RepositoryLocation>,
        format: &Format,
    ) -> CliOutput {
        match format {
            Format::Fancy => CliOutput::Stderr(
                generate_fancy_vertical_list(
                    format!("Repos for category '{}'", category),
                    repositories.iter().map(|repository| &repository.repository),
                )
                .into(),
            ),
            Format::Simple => CliOutput::Stdout(
                repositories
                    .iter()
                    .map(|repository| {
                        format!("{}/{}", &repository.category, &repository.repository)
                    })
                    .join(" ")
                    .into(),
            ),
        }
    }

    fn generate_output_all_repositories<T: Iterator<Item = RepositoryLocation>>(
        format: &Format,
        categories: T,
    ) -> CliOutput {
        let mut previous_category = String::new();
        let mut result = String::new();

        for location in categories
            .sorted()
            .map(Some)
            .chain([None].into_iter())
            .tuple_windows::<(_, _)>()
        {
            let (
                RepositoryLocation {
                    category,
                    repository,
                },
                next_category,
            ) = match location {
                (Some(location), None) => (location, String::new()),
                (Some(location), Some(RepositoryLocation { category, .. })) => {
                    (location, AsRef::<String>::as_ref(&category).clone())
                }
                // TODO: This should never happen, log this in the future I guess
                (None, _) => break,
            };

            if matches!(format, Format::Fancy) && category.as_ref() != previous_category {
                previous_category = AsRef::<String>::as_ref(&category).clone();
                result += format!("┌ Repositories for category '{}'\n│\n", category).as_str();
            };

            let additional = match format {
                Format::Fancy => {
                    if next_category.is_empty() {
                        format!("└─ {}", repository)
                    } else if category.as_ref() != next_category {
                        format!("└─ {}\n\n", repository)
                    } else {
                        format!("├─ {}\n", repository)
                    }
                }
                Format::Simple => format!("{}/{} ", category, repository),
            };

            result += additional.as_str();
        }

        match format {
            Format::Fancy => CliOutput::Stderr(result.into()),
            Format::Simple => CliOutput::Stdout(result.into()),
        }
    }

    fn generate_output_category_name_only<'a, T>(categories: T, format: &Format) -> CliOutput
    where
        T: IntoIterator<Item = &'a String>,
    {
        match format {
            Format::Fancy => {
                CliOutput::Stderr(generate_fancy_vertical_list("Categories", categories).into())
            }
            Format::Simple => CliOutput::Stdout(categories.into_iter().join(" ").into()),
        }
    }

    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> CliResult {
        let output = match self {
            LsCommand {
                category: None,
                all: false,
                format,
            } => Self::generate_output_category_name_only(
                grass::dev::list_categories::<_, Vec<_>>(api)?.iter(),
                &format.clone().unwrap_or_default(),
            ),
            LsCommand {
                category: Some(category),
                all: false,
                format,
            } => Self::generate_output_repositories_for_category(
                category.clone(),
                grass::dev::list_repositories_in_category(api, category)?,
                &format.clone().unwrap_or_default(),
            ),
            LsCommand {
                category: None,
                all: true,
                format,
            } => Self::generate_output_all_repositories(
                &format.clone().unwrap_or_default(),
                grass::dev::list_all_repositories::<_, Vec<_>>(api)?.into_iter(),
            ),
            _ => {
                // TODO: Generate more specific output
                return Err(
                    Into::<anyhow::Error>::into(CliError::new("Invalid flags provided."))
                        .context("When running the command 'grass ls'"),
                );
            }
        };

        Ok(output.append_newline())
    }
}
