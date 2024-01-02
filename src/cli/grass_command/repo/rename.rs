use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Parser, Debug)]
pub struct RenameCommand {
    category: String,
    old_repository: String,
    new_repository: Arc<str>,
}

impl RenameCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        let old_location: (&str, &str) = (self.category.as_ref(), self.old_repository.as_ref());
        let new_location: (&str, &str) = (self.category.as_ref(), self.new_repository.as_ref());
        grass::dev::move_repository(api, old_location, new_location)?;

        Ok(())
    }
}
