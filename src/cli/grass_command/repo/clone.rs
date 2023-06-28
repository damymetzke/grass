use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::ApiStrategy, Api};

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
        let category = self.category.clone().map_or_else(
            || {
                select_selectable(grass::dev::list_categories::<_, Vec<_>>(api)?.as_slice())
                    .cloned()
            },
            Ok,
        )?;

        grass::dev::clone_repository_default(api, category, &self.remote)?;
        Ok(())
    }
}
