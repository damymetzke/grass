use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CleanCommand {
    category: String,
    repository: String,
}

impl CleanCommand {
    pub fn handle(&self) -> Result<()> {
        let user_config = grass::dev::config::load_user_config()?;

        Ok(grass::dev::clean_repository(
            &user_config,
            &self.category,
            &self.repository,
        )?)
    }
}
