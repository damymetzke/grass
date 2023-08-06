mod local;
mod mock;
use thiserror::Error;

pub use local::LocalGitStrategy;
pub use mock::MockGitStrategy;

use crate::dev::{error::GrassError, public::api::RepositoryLocation};

use super::alias::AliasStrategyError;

/// Error returned from methods in `GitStrategy`[^strategy].
///
/// # Todo:
///
/// - [ ] Change to `context` and `reason` fields.
///
/// [^strategy]: [crate::dev::strategy::git::GitStrategy]
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

/// Alias for results in methods from `GitStrategy`[^strategy]
///
/// [^strategy]: [crate::dev::strategy::git::GitStrategy]
pub type Result<T> = std::result::Result<T, GitStrategyError>;

/// Describes the status of a repository.
///
/// The status is related to whether or not there are changes.
#[derive(Debug, PartialEq, Eq)]
pub enum RepositoryChangeStatus {
    /// All changes have been committed.
    UpToDate,
    /// No repository has been initialized.
    NoRepository,
    /// A repository has been initialized, but there are uncommitted changes.
    ///
    /// `num_changes` is not strongly defined, this number may change between versions.
    /// It has no real meaning, and should only be used for generic estimates.
    UncommittedChanges { num_changes: usize },
    /// This repository has an unknown status.
    ///
    /// This is only applicable if the unkown status is within the expected behavior of the
    /// implementation.
    /// For example, if an implementation has to synchronize in the background, then it may result
    /// in an unkown status.
    ///
    /// This means that unknown doesn't mean something is wrong, although it may be.
    /// If the status is unknown due to an error, use `Error`[^error] instead.
    ///
    /// [^error]: [crate::dev::strategy::git::RepositoryChangeStatus::Error]
    Unknown,
}

/// Describes the status of a repository.
///
/// The status is related to whether or not there are changes.
#[derive(Debug, PartialEq, Eq)]
pub enum RepositoryChangeStatusWithError {
    /// All changes have been committed.
    UpToDate,
    /// No repository has been initialized.
    NoRepository,
    /// A repository has been initialized, but there are uncommitted changes.
    ///
    /// `num_changes` is not strongly defined, this number may change between versions.
    /// It has no real meaning, and should only be used for generic estimates.
    UncommittedChanges { num_changes: usize },
    /// The status is unkown due to an error.
    Error { reason: GrassError },
    /// This repository has an unknown status.
    ///
    /// This is only applicable if the unkown status is within the expected behavior of the
    /// implementation.
    /// For example, if an implementation has to synchronize in the background, then it may result
    /// in an unkown status.
    ///
    /// This means that unknown doesn't mean something is wrong, although it may be.
    /// If the status is unknown due to an error, use `Error`[^error] instead.
    ///
    /// [^error]: [crate::dev::strategy::git::RepositoryChangeStatus::Error]
    Unknown,
}

/// Strategy for all git operations.
///
/// Apply git operations on a repository.
/// Operations can return information, or mutate the repository.
///
/// # Implementations
///
/// | Strategy                                      | Description                   |
/// | :-------------------------------------------- | :---------------------------- |
/// | [crate::dev::strategy::git::LocalGitStrategy] | Access local git repositories |
/// | [crate::dev::strategy::git::MockGitStrategy]  | Mocking implementation        |
///
/// # See
///
/// - [crate::dev::strategy::git::SupportsGit]
pub trait GitStrategy {
    /// Clean the repository from files that are explicitely ignored.
    ///
    /// This will **not** delete uncommitted changes.
    /// Rather it will clean files which exists, but are ignored.
    /// It's purpose then is to clean out optional files,
    /// for example to optimize disk space.
    ///
    /// # Example
    ///
    /// ```rust
    /// use grass::dev::strategy::git::{GitStrategy, GitStrategyError, MockGitStrategy};
    /// let strategy = MockGitStrategy;
    ///
    /// fn test_strategy<T: GitStrategy>(strategy: &T) {
    ///     assert_eq!(strategy.clean(("all_good", "first")), Ok(()));
    ///
    ///     assert!(matches!(
    ///         strategy.clean(("with_error", "first")),
    ///         Err(GitStrategyError::RepositoryError { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.clean(("with_error", "second")),
    ///         Err(GitStrategyError::FileSystemError { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.clean(("missing", "first")),
    ///         Err(GitStrategyError::RepositoryNotFound { .. })
    ///     ));
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn clean<T>(&self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>;

    /// Clone a remote repository.
    ///
    /// # Todo:
    ///
    /// - [ ] Support authentication:
    ///     - [ ] SSH
    ///     - [ ] Password
    ///     - [ ] PAT
    ///
    /// # Example
    ///
    /// ```rust
    /// use grass::dev::strategy::git::{GitStrategy, GitStrategyError, MockGitStrategy};
    /// let strategy = MockGitStrategy;
    ///
    /// fn test_strategy<T: GitStrategy>(strategy: &T) {
    ///     assert_eq!(strategy.clone(("all_good", "new"), "good_remote"), Ok(()));
    ///
    ///     assert!(matches!(
    ///         strategy.clone(("all_good", "first"), "good_remote"),
    ///         Err(GitStrategyError::RepositryExists { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.clone(("missing", "first"), "good_remote"),
    ///         Err(GitStrategyError::RepositoryNotFound { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.clone(("all_good", "new"), "no_access"),
    ///         Err(GitStrategyError::RemoteAuthenticationError { .. })
    ///     ));
    ///
    ///     assert!(matches!(
    ///         strategy.clone(("all_good", "new"), "bad_response"),
    ///         Err(GitStrategyError::RemoteFetchError { .. })
    ///     ));
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn clone<T, U>(&self, repository: T, remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>;

    /// Get the change status for a repository.
    ///
    /// # Example
    ///
    /// ```rust
    /// use grass::dev::strategy::git::{GitStrategy, MockGitStrategy, RepositoryChangeStatus};
    /// let strategy = MockGitStrategy;
    ///
    /// fn test_strategy<T: GitStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.get_changes(("with_changes", "first")),
    ///         Ok(RepositoryChangeStatus::UpToDate)
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.get_changes(("with_changes", "second")),
    ///         Ok(RepositoryChangeStatus::NoRepository)
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.get_changes(("with_changes", "third")),
    ///         Ok(RepositoryChangeStatus::UncommittedChanges { num_changes: 9 })
    ///     );
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn get_changes<T>(&self, repository: T) -> Result<RepositoryChangeStatus>
    where
        T: Into<RepositoryLocation>;
}

/// Implement to support actions requiring the `GitStrategy`[^strategy].
///
/// # See
///
/// - [crate::dev::strategy]
/// - [crate::dev::Api]
///
/// [^strategy]: [crate::dev::strategy::git::GitStrategy]
pub trait SupportsGit {
    type Git: GitStrategy;

    fn get_git_strategy(&self) -> &Self::Git;
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

impl From<AliasStrategyError> for GitStrategyError {
    fn from(value: AliasStrategyError) -> Self {
        match value {
            AliasStrategyError::UnkownError { context, reason } => GitStrategyError::UnknownError {
                message: context,
                reason,
            },
            AliasStrategyError::CategoryNotFound { context, reason } => {
                GitStrategyError::RepositoryNotFound {
                    message: context,
                    reason,
                }
            }
        }
    }
}

impl<T: Into<GrassError>> From<std::result::Result<RepositoryChangeStatus, T>>
    for RepositoryChangeStatusWithError
{
    fn from(value: std::result::Result<RepositoryChangeStatus, T>) -> Self {
        match value {
            Ok(change_status) => change_status.into(),
            Err(error) => RepositoryChangeStatusWithError::Error {
                reason: error.into(),
            },
        }
    }
}

impl From<RepositoryChangeStatus> for RepositoryChangeStatusWithError {
    fn from(value: RepositoryChangeStatus) -> Self {
        match value {
            RepositoryChangeStatus::UpToDate => RepositoryChangeStatusWithError::UpToDate,
            RepositoryChangeStatus::NoRepository => RepositoryChangeStatusWithError::NoRepository,
            RepositoryChangeStatus::UncommittedChanges { num_changes } => {
                RepositoryChangeStatusWithError::UncommittedChanges { num_changes }
            }
            RepositoryChangeStatus::Unknown => RepositoryChangeStatusWithError::Unknown,
        }
    }
}
