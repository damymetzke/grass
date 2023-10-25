use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Parser, Debug, Clone)]
pub struct ListCommand;

impl ListCommand {
    pub fn handle<T>(&self, _api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        eprintln!("This view is temporary, and will show the current configuration in the future");
        eprintln!();

        eprintln!("base_dir => Home directory where repositories are stored");

        eprintln!("category.[category_name] => A category of repositories");
        eprintln!("category.[category_name].[[alias]] => An alias to the category");
        Ok(())
    }
}
