use std::fs;

use crate::dev::{
    config::GrassConfig,
    public::api::RepositoryLocation,
    strategy::{
        discovery::DiscoveryStrategyError,
        path::{PathStrategy, PathStrategyError},
    },
};

use super::{BoxedIterator, DiscoveryExists, DiscoveryStrategy, Result};

pub struct LocalDiscoveryStrategy<'a, T>
where
    T: PathStrategy,
{
    config: &'a GrassConfig,
    path_strategy: &'a T,
}

impl<'a, T> LocalDiscoveryStrategy<'a, T>
where
    T: PathStrategy,
{
    pub fn new(config: &'a GrassConfig, path_strategy: &'a T) -> Self {
        LocalDiscoveryStrategy {
            config,
            path_strategy,
        }
    }
}

impl<'a, T> DiscoveryStrategy for LocalDiscoveryStrategy<'a, T>
where
    T: PathStrategy,
{
    fn check_repository_exists<U>(&self, repository: U) -> Result<DiscoveryExists>
    where
        U: Into<RepositoryLocation>,
    {
        let repository_dir = self.path_strategy.get_directory(repository)?;

        match repository_dir.is_dir() {
            true => Ok(DiscoveryExists::Exists),
            false => Ok(DiscoveryExists::RepositoryNotFound),
        }
    }

    fn check_category_exists<U>(&self, category: U) -> Result<DiscoveryExists>
    where
        U: AsRef<str>,
    {
        match self.config.get_by_category(category) {
            Some(_) => Ok(DiscoveryExists::Exists),
            None => Ok(DiscoveryExists::CategoryNotFound),
        }
    }

    fn list_repositories_in_category<U>(
        &self,
        category: U,
    ) -> Result<BoxedIterator<Result<RepositoryLocation>>>
    where
        U: AsRef<str>,
    {
        let category = self.config.get_by_category(category).ok_or(
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
                category: category.name.clone().into(),
                repository,
            }))
        })))
    }

    fn list_categories<U>(&self) -> Result<U>
    where
        U: FromIterator<String>,
    {
        Ok(self.config.category.keys().cloned().collect())
    }
}

impl From<PathStrategyError> for DiscoveryStrategyError {
    fn from(value: PathStrategyError) -> Self {
        match value {
            PathStrategyError::RepositoryNotFound { context, reason } => {
                DiscoveryStrategyError::CategoryNotFound { context, reason }
            }
            PathStrategyError::FileDoesNotExist { context, reason } => {
                DiscoveryStrategyError::FilesystemError { context, reason }
            }
            PathStrategyError::Unknown { context, reason } => {
                DiscoveryStrategyError::UnknownError { context, reason }
            }
        }
    }
}
