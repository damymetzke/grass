use anyhow::Result;
use clap::Parser;
use grass::dev::{config, strategy::api::ApiStrategy, Api};

use crate::facades::dialoguer::select_selectable;

#[derive(Parser, Debug)]
pub struct CloneCommand {
    remote: String,
    category: Option<String>,
}

impl CloneCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: ApiStrategy,
    {
        let user_config = config::load_user_config()?;
        let category = self.category.clone().map_or_else(
            || select_selectable(&grass::dev::list_categories(&user_config)).cloned(),
            Ok,
        )?;

        grass::dev::clone_repository_default(api, category, &self.remote)?;
        Ok(())
    }
}
