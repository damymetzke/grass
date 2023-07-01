use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Parser, Debug)]
pub struct CleanCommand {
    category: String,
    repository: String,
}

impl CleanCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        Ok(grass::dev::clean_repository(
            api,
            (&self.category, &self.repository),
        )?)
    }
}
