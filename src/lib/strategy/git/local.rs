use git2::Repository;

use crate::config::GrassConfig;

use super::{GitStrategy, GitStrategyError, RepositoryChangeStatus, RepositoryLocation, Result};

pub struct LocalGitStrategy<'a> {
    config: &'a GrassConfig,
}

impl<'a> LocalGitStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        Self { config }
    }
}

impl<'a> GitStrategy for LocalGitStrategy<'a> {
    fn clean<T>(&self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository_path = crate::dev::get_repository_path(
            self.config,
            repository.category,
            repository.repository,
        )
        .ok_or_else(|| GitStrategyError::RepositoryNotFound {
            message: "Cannot not find repository".into(),
        })?;

        let repository =
            Repository::open(&repository_path).map_err(|error| match error.code() {
                git2::ErrorCode::NotFound => GitStrategyError::RepositoryNotFound {
                    message: error.to_string(),
                },
                _ => GitStrategyError::RepositoryError {
                    message: error.to_string(),
                },
            })?;

        let statuses =
            repository
                .statuses(None)
                .map_err(|error| GitStrategyError::RepositoryError {
                    message: error.to_string(),
                })?;

        let results: Vec<_> = statuses
            .iter()
            .filter_map(|entry| match entry.path() {
                Some(file_path) if entry.status().is_ignored() => Some(file_path.to_owned()),
                _ => None,
            })
            .map(|file| {
                let path = repository_path.join(file);
                if path.is_dir() {
                    std::fs::remove_dir_all(path)
                } else {
                    std::fs::remove_file(path)
                }
            })
            .filter_map(std::result::Result::err)
            .map(|error| error.to_string())
            .collect();

        match results.len() {
            0 => Ok(()),
            _ => Err(GitStrategyError::FileSystemError {
                message: "Cannot clean repository".into(),
                reasons: results,
            }),
        }
    }

    fn clone<T, U>(&self, _repository: T, _remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>,
    {
        todo!()
    }

    fn get_changes<T>(&self, _repository: T) -> Result<RepositoryChangeStatus>
    where
        T: Into<RepositoryLocation>,
    {
        todo!()
    }
}
