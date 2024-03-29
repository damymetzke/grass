mod local;
mod mock;

use std::path::PathBuf;

use thiserror::Error;

use crate::{dev::RepositoryLocation, support_strategy};

pub use local::LocalPathStrategy;
pub use mock::MockPathStrategy;

#[derive(Error, Debug, PartialEq, Eq, Hash)]
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
    /// ```rust
    /// use grass::dev::strategy::path::{MockPathStrategy, PathStrategy, PathStrategyError};
    /// use std::path::PathBuf;
    /// let strategy = MockPathStrategy;
    /// fn test_strategy<T: PathStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.get_containing_directory(("all_good", "first")),
    ///         Ok(PathBuf::from("/home/example/repositories/all_good"))
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.get_containing_directory(("with_changes", "does_not_exist")),
    ///         Ok(PathBuf::from("/home/example/repositories/with_changes"))
    ///     );
    ///
    ///     assert!(matches!(
    ///         strategy.get_containing_directory(("does_not_exist", "third")),
    ///         Err(PathStrategyError::RepositoryNotFound { .. })
    ///     ));
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn get_containing_directory<T>(&self, repository: T) -> Result<PathBuf>
    where
        T: Into<RepositoryLocation>;

    /// Get the directory for a repository
    ///
    /// This will also work if the path doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use grass::dev::strategy::path::{MockPathStrategy, PathStrategy, PathStrategyError};
    /// use std::path::PathBuf;
    /// let strategy = MockPathStrategy;
    /// fn test_strategy<T: PathStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.get_directory(("all_good", "first")),
    ///         Ok(PathBuf::from("/home/example/repositories/all_good/first"))
    ///     );
    ///
    ///     assert!(matches!(
    ///         strategy.get_directory(("all_good", "does_not_exist")),
    ///         Err(PathStrategyError::RepositoryNotFound { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.get_directory(("does_not_exist", "third")),
    ///         Err(PathStrategyError::RepositoryNotFound { .. })
    ///     ));
    /// }
    /// test_strategy(&strategy);
    /// ```
    fn get_directory<T>(&self, repository: T) -> Result<PathBuf>
    where
        T: Into<RepositoryLocation>;
}

support_strategy!(SupportsPath, get_path_strategy, PathStrategy);
