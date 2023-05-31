mod local;

use std::collections::HashMap;

use thiserror::Error;

pub use local::LocalGitStrategy;

#[derive(Error, Debug)]
pub enum GitStrategyError {
    #[error("Cannot find repository:\n{message}")]
    RepositoryNotFound { message: String },
    #[error("There is a problem with the repository:\n{message}")]
    RepositoryError { message: String },
    #[error("There is a problem fetching the remote '{remote}':\n{message}")]
    RemoteFetchError { message: String, remote: String },
    #[error("There is a problem authenticating for remote '{remote}':\n{message}")]
    RemoteAuthenticationError { message: String, remote: String },
    #[error("The remote '{remote}' is unavailable:\n{message}")]
    RemoteUnavailable { message: String, remote: String },
    #[error("There is a problem accessing the file system:\n{message}")]
    FileSystemError { message: String, reasons: Vec<std::io::Error>},
    #[error("There is a problem:\n{message}")]
    UnknownError { message: String },
}

pub type Result<T> = std::result::Result<T, GitStrategyError>;

pub struct RepositoryLocation {
    pub category: String,
    pub repository: String,
}

pub enum RepositoryChangeStatus {
    UpToDate,
    NoRepository,
    FilesChanged {
        added: usize,
        deleted: usize,
        changed: usize,
        total: usize,
    },
}

pub trait GitStrategy {
    fn clean<T>(self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>;

    fn clone<T, U>(self, repository: T, remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>;

    fn get_changes<T>(self, repository: T) -> Result<HashMap<String, RepositoryChangeStatus>>
    where
        T: Into<RepositoryLocation>;
}

impl RepositoryLocation {
    pub fn new<T, U>(category: T, repository: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        RepositoryLocation {
            category: category.into(),
            repository: repository.into(),
        }
    }
}

impl<T, U> From<(T, U)> for RepositoryLocation
where
    T: Into<String>,
    U: Into<String>,
{
    fn from((category, repository): (T, U)) -> Self {
        Self {
            category: category.into(),
            repository: repository.into(),
        }
    }
}
