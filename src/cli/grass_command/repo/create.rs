use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Parser, Debug)]
pub struct CreateCommand {
    category: String,
    repository: String,
}

impl CreateCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        let location: (&str, &str) = (self.category.as_ref(), self.repository.as_ref());
        grass::dev::create_repository(api, location)?;

        Ok(())
    }
}
