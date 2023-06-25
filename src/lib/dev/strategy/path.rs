mod mock;

use std::path::PathBuf;

use thiserror::Error;

use crate::dev::RepositoryLocation;

pub use mock::MockPathStrategy;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PathStrategyError {
    #[error("Cannot find repository:\nContext: {context}\nReason: {reason}")]
    RepositoryNotFound { context: String, reason: String },

    #[error("A file does not exist:\nContext: {context}\nReason: {reason}")]
    FileDoesNotExist { context: String, reason: String },

    #[error("There is a problem:\nContext: {context}\nReason: {reason}")]
    Unknown { context: String, reason: String },
}

pub type Result<T> = std::result::Result<T, PathStrategyError>;

pub trait PathStrategy {
    /// Get the containing directory for a repository
    ///
    /// This can work even when the repository doesn't exist (yet).
    /// So this can be used to get the directory to move into, for example.
    ///
    /// Grass makes no guarentees that all repositories will be contained in the same directory.
    /// You cannot assume that other repositories in the same category are contained in the same
    /// directory.
    ///
    /// # Example
    ///
    /// TODO: Add when mock strategy has been created
    fn get_containing_directory<T>(&self, repository: T) -> Result<PathBuf>
    where
        T: Into<RepositoryLocation>;

    /// Get the directory for a repository
    ///
    /// This will fail if the repository, or the path, doesn't exist.
    ///
    /// # Example
    ///
    /// TODO: Add when mock strategy has been created
    fn get_directory<T>(&self, repository: T) -> Result<PathBuf>
    where
        T: Into<RepositoryLocation>;
}
