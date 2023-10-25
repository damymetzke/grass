use anyhow::Result;
use clap::{Parser, ValueEnum};
use grass::dev::{strategy::api::SupportsAll, Api};

#[derive(Debug, Clone, ValueEnum)]
enum ConfigKey {
    #[value(name = "base_dir")]
    Basedir,
    #[value(name = "category.*")]
    CategoryKey,
    #[value(name = "category.*.alias")]
    CategoryKeyAlias,
}

#[derive(Parser, Debug, Clone)]
pub struct ExplainCommand {
    config_key: ConfigKey,
}

impl ExplainCommand {
    pub fn handle<T>(&self, _api: &Api<T>) -> Result<()>
    where
        T: SupportsAll,
    {
        match self.config_key {
            ConfigKey::Basedir => eprintln!("The base directory where repositories are stored"),
            ConfigKey::CategoryKey => eprintln!("The name of a category"),
            ConfigKey::CategoryKeyAlias => eprintln!("Aliases for the category"),
        };
        Ok(())
    }
}
