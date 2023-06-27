mod local;
mod mock;

use thiserror::Error;

pub use local::LocalGitStrategy;
pub use mock::MockGitStrategy;

use crate::dev::public::api::RepositoryLocation;

// TODO: Change to `context` and `reason` fields.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum GitStrategyError {
    #[error("Cannot find repository:\n{message}\nReason: {reason}")]
    RepositoryNotFound { message: String, reason: String },
    #[error("There is a problem with the repository:\n{message}\nReason: {reason}")]
    RepositoryError { message: String, reason: String },
    #[error("The repository already exists:\n{message}\nReason: {reason}")]
    RepositryExists { message: String, reason: String },
    #[error("There is a problem fetching a remote:\n{message}\nReason: {reason}")]
    RemoteFetchError { message: String, reason: String },
    #[error("There is a problem authenticating for a remote:\n{message}\nReason: {reason}")]
    RemoteAuthenticationError { message: String, reason: String },
    #[error("There is a problem accessing the file system:\n{message}\nReason: {reason}")]
    FileSystemError {
        message: String,
        reason: String,
        reasons: Vec<String>,
    },
    #[error("There is a problem:\n{message}\nReason: {reason}")]
    UnknownError { message: String, reason: String },
}

pub type Result<T> = std::result::Result<T, GitStrategyError>;

#[derive(Debug, PartialEq, Eq)]
pub enum RepositoryChangeStatus {
    UpToDate,
    NoRepository,
    FilesChanged { num_changes: usize },
}

pub trait GitStrategy {
    fn clean<T>(&self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>;

    fn clone<T, U>(&self, repository: T, remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>;

    fn get_changes<T>(&self, repository: T) -> Result<RepositoryChangeStatus>
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

impl GitStrategyError {
    pub fn with_message<T>(self, message: T) -> Self
    where
        T: Into<String>,
    {
        use GitStrategyError::*;

        let message = message.into();
        match self {
            RepositoryNotFound { reason, .. } => RepositoryNotFound { message, reason },
            RepositoryError { reason, .. } => RepositoryError { message, reason },
            RepositryExists { reason, .. } => RepositryExists { message, reason },
            RemoteFetchError { reason, .. } => RemoteFetchError { message, reason },
            RemoteAuthenticationError { reason, .. } => {
                RemoteAuthenticationError { message, reason }
            }
            FileSystemError {
                reason, reasons, ..
            } => FileSystemError {
                message,
                reason,
                reasons,
            },
            UnknownError { reason, .. } => UnknownError { message, reason },
        }
    }
}
