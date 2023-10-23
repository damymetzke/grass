use anyhow::Result;
use clap::{Parser, ValueEnum};
use grass::dev::{
    config,
    iterator::{
        location::LocationIterExtensions,
        location_and_change_status::LocationAndChangeStatusIterExtensions,
    },
    list_all_repositories, list_repositories_in_category,
    strategy::api::SupportsAll,
    Api,
};

use crate::error::CliError;

#[derive(ValueEnum, Debug, Clone, Default)]
enum Format {
    #[default]
    Fancy,
    Simple,
}

#[derive(Parser, Debug)]
pub struct ChangesCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
    #[clap(long)]
    format: Option<Format>,
}

impl ChangesCommand {
    pub fn handle<T: SupportsAll>(&self, api: &Api<T>) -> Result<()> {
        // TODO: Handle error
        let user_config = config::load_user_config().unwrap();

        let mut repositories: Box<[_]> = match self {
            Self {
                category: Some(category),
                all: false,
                format,
            } => {
                let repositories: Vec<_> = list_repositories_in_category(api, category)?;
                repositories
                    .into_iter()
                    .with_change_status(api)
                    .uncommitted_changes_only()
                    .collect()
            }
            Self {
                category: None,
                all: true,
                format,
            } => {
                let repositories: Vec<_> = list_all_repositories(api)?;
                repositories
                    .into_iter()
                    .with_change_status(api)
                    .uncommitted_changes_only()
                    .collect()
            }
            Self {
                category: Some(_),
                all: true,
                format: _,
            } => return Err(CliError::new("Received incompatible category with all flag").into()),
            Self {
                category: None,
                all: false,
                format: _,
            } => return Err(CliError::new("Provide either a category, or the --all flag").into()),
        };

        repositories.sort();
        eprintln!("{:?}", repositories);
        Ok(())
    }
}
