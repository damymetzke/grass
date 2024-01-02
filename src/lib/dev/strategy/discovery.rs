mod local;
mod mock;

use thiserror::Error;

use crate::dev::public::api::RepositoryLocation;

pub use local::LocalDiscoveryStrategy;
pub use mock::MockDiscoveryStrategy;

use super::alias::AliasStrategyError;

/// Error returned by methods of [DiscoveryStrategy]
///
/// Each variant has 2 fields:
///
/// - `context`: What action was attempted.
/// - `reason`: What went wrong, often this is provided by errors generated from crates.
///
/// See: [crate::dev::strategy::discovery::DiscoveryStrategy]
#[derive(Error, Debug, PartialEq, Eq, Hash)]
pub enum DiscoveryStrategyError {
    #[error("Cannot find repository:\nContext: {context}\nReason: {reason}")]
    CategoryNotFound { context: String, reason: String },
    #[error("There is a problem accessing the file system:\nContext: {context}\nReason: {reason}")]
    FilesystemError { context: String, reason: String },
    #[error("There is a problem:\nContext: {context}\nReason: {reason}")]
    UnknownError { context: String, reason: String },
    #[error("Repository already exists:\nContext: {context}\nReason: {reason}")]
    RepositoryExists { context: Box<str>, reason: Box<str> },
    #[error("Repository does not exist:\nContext: {context}\nReason: {reason}")]
    RepositoryDoesNotExist { context: Box<str>, reason: Box<str> },
}

/// Methods of [DiscoveryStrategy] return this alias
///
/// See: [crate::dev::strategy::discovery::DiscoveryStrategy]
pub type Result<T> = std::result::Result<T, DiscoveryStrategyError>;

/// Indicates wheter a respository/category exists
#[derive(Debug, PartialEq, Eq)]
pub enum DiscoveryExists {
    Exists,
    /// The category exists, but the repository does not
    RepositoryNotFound,
    CategoryNotFound,
}

pub type BoxedIterator<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

/// Strategy for finding repositories and categories
///
/// The exact behavior of this strategy can vary.
/// By design, this concept is abstract.
/// The goal of the discovery strategy, is to provide a way to discover which options there are.
///
/// In the case of [LocalDiscoveryStrategy][^local], 2 things are considered:
///
/// - The provided configuration.
/// - Folders in the file system.
///
/// # See
///
/// [crate::dev::strategy::discovery::SupportsDiscovery]
///
/// [^local]: [crate::dev::strategy::discovery::LocalDiscoveryStrategy]
pub trait DiscoveryStrategy {
    ///
    /// # Return
    ///
    /// | Variant                               | Meaning                                         |
    /// | :------------------------------------ | :---------------------------------------------- |
    /// | [DiscoveryExists::Exists]             | The repository exists                           |
    /// | [DiscoveryExists::RepositoryNotFound] | The category exists, but the repository doesn't |
    /// | [DiscoveryExists::CategoryNotFound]   | The category doesn't exist                      |
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::discovery::{
    ///     DiscoveryStrategy, MockDiscoveryStrategy, DiscoveryExists
    /// };
    /// # let strategy = MockDiscoveryStrategy;
    /// fn test_strategy<T: DiscoveryStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.check_repository_exists(("all_good", "first")),
    ///         Ok(DiscoveryExists::Exists),
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.check_repository_exists(("all_good", "missing")),
    ///         Ok(DiscoveryExists::RepositoryNotFound),
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.check_repository_exists(("missing", "first")),
    ///         Ok(DiscoveryExists::CategoryNotFound),
    ///     );
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn check_repository_exists<T>(&self, repository: T) -> Result<DiscoveryExists>
    where
        T: Into<RepositoryLocation>;

    ///
    /// # Return
    ///
    /// | Variant                             | Meaning                    |
    /// | :---------------------------------- | :------------------------- |
    /// | [DiscoveryExists::Exists]           | The repository exists      |
    /// | [DiscoveryExists::CategoryNotFound] | The category doesn't exist |
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::discovery::{
    ///     DiscoveryStrategy, MockDiscoveryStrategy, DiscoveryExists
    /// };
    /// # let strategy = MockDiscoveryStrategy;
    /// fn test_strategy<T: DiscoveryStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.check_category_exists("all_good"),
    ///         Ok(DiscoveryExists::Exists),
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.check_category_exists("missing"),
    ///         Ok(DiscoveryExists::CategoryNotFound),
    ///     );
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    fn check_category_exists<T>(&self, category: T) -> Result<DiscoveryExists>
    where
        T: AsRef<str>;

    /// Get an iterator over all repositories in a category
    ///
    /// Discovery depends on the implementation.
    /// In the case of [LocalDiscoveryStrategy][^1], this means all folders in a specified folder.
    ///
    /// The result will be an iterator over the objects using a `Result`.
    /// Individual repositories may fail for one reason or another.
    /// You are expected to filter these out if you want to ignore them.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use grass::dev::strategy::discovery::{
    ///     DiscoveryStrategy, MockDiscoveryStrategy, DiscoveryStrategyError
    /// };
    /// # use grass::dev::RepositoryLocation;
    /// # let strategy = MockDiscoveryStrategy;
    /// fn test_strategy<T: DiscoveryStrategy>(strategy: &T) {
    ///     let list_repositories_result: Vec<_> = strategy
    ///         .list_repositories_in_category("all_good")
    ///         .unwrap()
    ///         .collect();
    ///
    ///     assert_eq!(
    ///         list_repositories_result,
    ///         vec![
    ///             Ok(RepositoryLocation {
    ///                 category: "all_good".into(),
    ///                 repository: "first".into(),
    ///             }),
    ///             Ok(RepositoryLocation {
    ///                 category: "all_good".into(),
    ///                 repository: "second".into(),
    ///             }),
    ///             Ok(RepositoryLocation {
    ///                 category: "all_good".into(),
    ///                 repository: "third".into(),
    ///             }),
    ///         ],
    ///     );
    ///
    ///     assert!(matches!(
    ///         strategy.list_repositories_in_category("missing"),
    ///         Err(DiscoveryStrategyError::CategoryNotFound{..}),
    ///     ));
    /// }
    ///
    /// test_strategy(&strategy);
    /// ```
    ///
    /// [^1]: [crate::dev::strategy::discovery::LocalDiscoveryStrategy]
    fn list_repositories_in_category<T>(
        &self,
        category: T,
    ) -> Result<BoxedIterator<Result<RepositoryLocation>>>
    where
        T: AsRef<str>;

    /// Retrieve a list of all categories
    ///
    /// The implementation can differ per strategy.
    /// In the case of [LocalDiscoveryStrategy][^1], a configuration file is used.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::discovery::{
    /// # DiscoveryStrategy, MockDiscoveryStrategy, DiscoveryStrategyError
    /// # };
    /// # let strategy = MockDiscoveryStrategy;
    /// fn test_strategy<T: DiscoveryStrategy>(strategy: &T) {
    ///     let list_categories_result: Vec<_> = strategy
    ///         .list_categories()
    ///         .unwrap();
    ///
    ///     assert_eq!(
    ///         list_categories_result,
    ///         vec!["all_good", "with_changes", "with_error"],
    ///     )
    /// }
    ///
    /// test_strategy(&strategy)
    /// ```
    ///
    /// [^1]: [crate::dev::strategy::discovery::LocalDiscoveryStrategy]
    fn list_categories<T>(&self) -> Result<T>
    where
        T: FromIterator<String>;

    fn create_repository(&self, location: RepositoryLocation) -> Result<()>;

    fn move_repository(&self, old_location: RepositoryLocation, new_location: RepositoryLocation) -> Result<()>;
}

pub trait SupportsDiscovery {
    type Discovery: DiscoveryStrategy;

    fn get_discovery_strategy(&self) -> &Self::Discovery;
}

impl From<AliasStrategyError> for DiscoveryStrategyError {
    fn from(value: AliasStrategyError) -> Self {
        match value {
            AliasStrategyError::UnkownError { context, reason } => {
                DiscoveryStrategyError::UnknownError { context, reason }
            }
            AliasStrategyError::CategoryNotFound { context, reason } => {
                DiscoveryStrategyError::CategoryNotFound { context, reason }
            }
        }
    }
}

impl From<std::io::Error> for DiscoveryStrategyError {
    fn from(value: std::io::Error) -> Self {
        Self::FilesystemError {
            context: "".into(),
            reason: value.to_string(),
        }
    }
}
