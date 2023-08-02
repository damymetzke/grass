use anyhow::Result;
use clap::Parser;
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Parser, Debug)]
/// Print the path of a category root, or a repository
///
/// The output will be a single string with no trailing newline.
pub struct PathCommand {
    /// The category to display, can be an alias
    category: String,
    /// The repository to display
    repository: String,
}

impl PathCommand {
    pub fn handle<T>(&self, api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        let path = grass::dev::get_repository_path_next(api, (&self.category, &self.repository))?;

        print!("{}", path.to_str().unwrap_or_default());
        Ok(())
    }
}
