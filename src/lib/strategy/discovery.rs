mod local;
mod mock;

use thiserror::Error;

use crate::public::api::RepositoryLocation;

pub use local::LocalDiscoveryStrategy;
pub use mock::MockDiscoveryStrategy;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DiscoveryStrategyError {
    #[error("Cannot find repository:\nContext: {context}\nReason: {reason}")]
    CategoryNotFound { context: String, reason: String },
    #[error("There is a problem:\nContext: {context}\nReason: {reason}")]
    UnknownError { context: String, reason: String },
}

pub type Result<T> = std::result::Result<T, DiscoveryStrategyError>;

pub enum DiscoveryExistsResult {
    Exists,
    RepositoryNotFound,
    CategoryNotFound,
}

pub trait DiscoveryStrategy {
    fn check_repository_exists<T>(&self, repository: T) -> Result<DiscoveryExistsResult>
    where
        T: Into<RepositoryLocation>;

    fn check_category_exists<T>(&self, category: T) -> Result<DiscoveryExistsResult>
    where
        T: AsRef<str>;

    fn list_repositories_in_category<T, U>(&self, category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<RepositoryLocation>;

    fn list_categories<T>(&self) -> Result<T>
    where
        T: FromIterator<String>;
}
