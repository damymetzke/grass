mod local;
mod mock;

use thiserror::Error;

pub use local::LocalGitStrategy;
pub use mock::MockGitStrategy;

use crate::public::api::RepositoryLocation;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GitStrategyError {
    #[error("Cannot find repository:\n{message}")]
    RepositoryNotFound { message: String },
    #[error("There is a problem with the repository:\n{message}")]
    RepositoryError { message: String },
    #[error("The repository already exists:\n{message}")]
    RepositryExists { message: String },
    #[error("There is a problem fetching a remote:\n{message}")]
    RemoteFetchError { message: String },
    #[error("There is a problem authenticating for a remote:\n{message}")]
    RemoteAuthenticationError { message: String },
    #[error("There is a problem accessing the file system:\n{message}")]
    FileSystemError {
        message: String,
        reasons: Vec<String>,
    },
    #[error("There is a problem:\n{message}")]
    UnknownError { message: String },
}

pub type Result<T> = std::result::Result<T, GitStrategyError>;

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
    fn clean<T>(&self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>;

    fn clone<T, U>(&self, repository: T, remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>;

    fn clone_default<T, U>(&self, category: T, remote: U) -> Result<()>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let repository = &remote
            .as_ref()
            .split('/')
            .last()
            .unwrap_or("repository")
            .trim_end_matches(".git");

        self.clone((category.as_ref(), *repository), &remote)
    }

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
