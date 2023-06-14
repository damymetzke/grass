use std::fs;

use crate::{
    config::GrassConfig, dev::strategy::discovery::DiscoveryStrategyError,
    public::api::RepositoryLocation,
};

use super::{BoxedIterator, DiscoveryExistsResult, DiscoveryStrategy, Result};

pub struct LocalDiscoveryStrategy<'a> {
    config: &'a GrassConfig,
}

impl<'a> LocalDiscoveryStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalDiscoveryStrategy { config }
    }
}

impl<'a> DiscoveryStrategy for LocalDiscoveryStrategy<'a> {
    fn check_repository_exists<T>(&self, repository: T) -> Result<DiscoveryExistsResult>
    where
        T: Into<RepositoryLocation>,
    {
        let RepositoryLocation {
            category,
            repository,
        } = repository.into();

        let category_dir = match self.config.get_from_category_or_alias(category) {
            Some(category) => self.config.base_dir.join(&category.name),
            None => return Ok(DiscoveryExistsResult::CategoryNotFound),
        };

        let repository_dir = category_dir.join(repository);

        match repository_dir.is_dir() {
            true => Ok(DiscoveryExistsResult::Exists),
            false => Ok(DiscoveryExistsResult::RepositoryNotFound),
        }
    }

    fn check_category_exists<T>(&self, category: T) -> Result<DiscoveryExistsResult>
    where
        T: AsRef<str>,
    {
        match self.config.get_from_category_or_alias(category) {
            Some(_) => Ok(DiscoveryExistsResult::Exists),
            None => Ok(DiscoveryExistsResult::CategoryNotFound),
        }
    }

    fn list_repositories_in_category<T>(
        &self,
        category: T,
    ) -> Result<BoxedIterator<Result<RepositoryLocation>>>
    where
        T: AsRef<str>,
    {
        let category = self.config.get_from_category_or_alias(category).ok_or(
            DiscoveryStrategyError::CategoryNotFound {
                context: "When trying to ge tthe category from the configuration".into(),
                reason: "Category doesn't exist".into(),
            },
        )?;

        let base_dir = self.config.base_dir.join(&category.name);

        let directory =
            fs::read_dir(&base_dir).map_err(|error| DiscoveryStrategyError::FilesystemError {
                context: format!(
                "When trying to read the directory '{dir}' (expected to be a category directory)",
                dir = base_dir.to_str().unwrap_or("CANNOT DISPLAY DIRECTORY PATH")
            ),
                reason: error.to_string(),
            })?;

        Ok(Box::from(directory.into_iter().filter_map(move |entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    return Some(Err(DiscoveryStrategyError::FilesystemError {
                        context: "When reading unknown entry".into(),
                        reason: error.to_string(),
                    }))
                }
            };

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(error) => {
                    return Some(Err(DiscoveryStrategyError::FilesystemError {
                        context: format!(
                            "When retrieving metadata for entry '{name}'",
                            name = entry
                                .path()
                                .to_str()
                                .unwrap_or("CANNOT DISPLAY DIRECTORY PATH")
                        ),
                        reason: error.to_string(),
                    }))
                }
            };

            if !metadata.is_dir() {
                return None;
            };

            let entry_path = entry.path();

            let repository = match entry_path.file_name().ok_or_else(|| {
                DiscoveryStrategyError::FilesystemError {
                    context: format!(
                        "When retrieving directory name from path {path}",
                        path = entry
                            .path()
                            .to_str()
                            .unwrap_or("CANNOT_DISPLAY_DIRECTORY_PATH")
                    ),
                    reason: "No reason given".into(),
                }
            }) {
                Ok(repository) => repository,
                Err(error) => return Some(Err(error)),
            };

            let repository: String = match repository.to_str() {
                Some(repository) => repository,
                None => {
                    return Some(Err(DiscoveryStrategyError::FilesystemError {
                        context: format!(
                            "When retrieving directory name from path {path}",
                            path = entry
                                .path()
                                .to_str()
                                .unwrap_or("CANNOT_DISPLAY_DIRECTORY_PATH")
                        ),
                        reason: "No reason given".into(),
                    }))
                }
            }
            .into();

            Some(Ok(RepositoryLocation {
                category: category.name.clone(),
                repository,
            }))
        })))
    }

    fn list_categories<T>(&self) -> Result<T>
    where
        T: FromIterator<String>,
    {
        Ok(self.config.category.keys().cloned().collect())
    }
}
