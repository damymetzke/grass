use crate::public::api::RepositoryLocation;

use super::{
    BoxedIterator, DiscoveryExistsResult, DiscoveryStrategy, DiscoveryStrategyError, Result,
};

#[derive(Default)]
pub struct MockDiscoveryStrategy;

impl DiscoveryStrategy for MockDiscoveryStrategy {
    fn check_repository_exists<T>(&self, repository: T) -> Result<DiscoveryExistsResult>
    where
        T: Into<crate::public::api::RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_str(), repository.repository.as_str());

        match repository {
            ("all_good" | "with_changes" | "with_error", "first" | "second")
            | ("all_good" | "with_changes", "third") => Ok(DiscoveryExistsResult::Exists),
            ("all_good" | "with_changes" | "with_error", _) => {
                Ok(DiscoveryExistsResult::RepositoryNotFound)
            }
            _ => Ok(DiscoveryExistsResult::CategoryNotFound),
        }
    }

    fn check_category_exists<T>(&self, category: T) -> Result<DiscoveryExistsResult>
    where
        T: AsRef<str>,
    {
        match category.as_ref() {
            "all_good" | "with_changes" | "with_error" => Ok(DiscoveryExistsResult::Exists),
            _ => Ok(DiscoveryExistsResult::CategoryNotFound),
        }
    }

    fn list_repositories_in_category<T>(
        &self,
        category: T,
    ) -> Result<BoxedIterator<Result<RepositoryLocation>>>
    where
        T: AsRef<str>,
    {
        let to_ok = |location| Ok(location);
        let result = match category.as_ref() {
            "all_good" => [
                ("all_good", "first"),
                ("all_good", "second"),
                ("all_good", "third"),
            ]
            .iter()
            .map(RepositoryLocation::from)
            .map(to_ok),
            "with_changes" => [
                ("with_changes", "first"),
                ("with_changes", "second"),
                ("with_changes", "third"),
            ]
            .iter()
            .map(RepositoryLocation::from)
            .map(to_ok),
            "with_errors" => [("with_errors", "first"), ("with_errors", "second")]
                .iter()
                .map(RepositoryLocation::from)
                .map(to_ok),
            _ => {
                return Err(DiscoveryStrategyError::CategoryNotFound {
                    context: "When mocking".into(),
                    reason: "Category does not exist".into(),
                })
            }
        };

        Ok(Box::from(result))
    }

    fn list_categories<T>(&self) -> Result<T>
    where
        T: FromIterator<String>,
    {
        Ok(["all_good", "with_changes", "with_error"]
            .into_iter()
            .map(String::from)
            .collect())
    }
}
