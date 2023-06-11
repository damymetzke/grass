use thiserror::Error;

use crate::public::api::RepositoryLocation;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DiscoveryStrategyError {
    #[error("There is a problem:\n{message}\nReason: {reason}")]
    UnknownError { message: String, reason: String },
}

pub type Result<T> = std::result::Result<T, DiscoveryStrategyError>;

pub enum DiscoveryExistsResult {
    Exists,
    RepositoryNotFound,
    CategoryNotFound,
}

pub trait DiscoveryStrategy {
    fn check_repository_exists<T>(repository: T) -> Result<DiscoveryExistsResult>
    where
        T: Into<RepositoryLocation>;

    fn check_category_exists<T>(category: T) -> Result<DiscoveryExistsResult>
    where
        T: AsRef<str>;

    fn list_repositories_in_category<T, U>(category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<RepositoryLocation>;

    fn list_categories<T>() -> Result<T>
    where
        T: FromIterator<RepositoryLocation>;
}
