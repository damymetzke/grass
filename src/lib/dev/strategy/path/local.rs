use std::path::PathBuf;

use crate::dev::{config::GrassConfig, strategy::path::PathStrategyError, RepositoryLocation};

use super::PathStrategy;

pub struct LocalPathStrategy<'a> {
    config: &'a GrassConfig,
}

impl<'a> LocalPathStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalPathStrategy { config }
    }
}

impl<'a> PathStrategy for LocalPathStrategy<'a> {
    fn get_containing_directory<T>(&self, repository: T) -> super::Result<PathBuf>
    where
        T: Into<RepositoryLocation>,
    {
        let RepositoryLocation { category, .. } = repository.into();

        let result = match self.config.get_by_category(category) {
            Some(category) => self.config.base_dir.join(&category.name),
            None => {
                return Err(PathStrategyError::RepositoryNotFound {
                    context: "When getting the category from configuration.".into(),
                    reason: "Category not defined.".into(),
                })
            }
        };
        Ok(result)
    }

    fn get_directory<T>(&self, repository: T) -> super::Result<PathBuf>
    where
        T: Into<RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let category_directory = self.get_containing_directory(repository.clone())?;

        let RepositoryLocation { repository, .. } = repository;

        Ok(category_directory.join(repository))
    }
}
