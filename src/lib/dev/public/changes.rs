use crate::dev::{
    strategy::{
        alias::{AliasStrategy, SupportsAlias},
        discovery::SupportsDiscovery,
        git::{GitStrategy, GitStrategyError, RepositoryChangeStatus, SupportsGit},
    },
    Api, Category, RepositoryLocation,
};

use super::strategy::AccessApi;

/// Get the change status of a specific repository
///
/// # Example
///
/// ```rust
/// # use grass::dev::{Api, use_mock_strategy};
/// # use grass::dev::strategy::git::RepositoryChangeStatus;
/// # let api = use_mock_strategy();
/// let api: Api<_> = api;
///
/// let no_changes =
///     grass::dev::get_repository_change_status(&api, ("with_changes", "first"))
///         .unwrap();
/// let no_repository =
///     grass::dev::get_repository_change_status(&api, ("with_changes", "second"))
///         .unwrap();
/// let with_changes =
///     grass::dev::get_repository_change_status(&api, ("with_changes", "third"))
///         .unwrap();
///
/// assert_eq!(no_changes, RepositoryChangeStatus::UpToDate);
/// assert_eq!(no_repository, RepositoryChangeStatus::NoRepository);
/// assert_eq!(
///     with_changes,
///     RepositoryChangeStatus::UncommittedChanges { num_changes: 9 }
/// );
/// ```
pub fn get_repository_change_status<T, U>(
    api: &Api<T>,
    repository: U,
) -> Result<RepositoryChangeStatus, GitStrategyError>
where
    T: SupportsGit + SupportsAlias,
    U: Into<RepositoryLocation>,
{
    let api = api.get_strategy();

    api.get_git_strategy().get_changes(
        api.get_alias_strategy()
            .resolve_location_alias(repository)?,
    )
}

pub fn list_repositories_with_change_status<T>(api: &Api<T>)
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
{
    todo!()
}

pub fn list_repositories_with_uncommitted_changes<T>(api: &Api<T>)
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
{
    todo!()
}

pub fn list_repositories_with_change_status_in_category<T, U>(api: &Api<T>, category: U)
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
    U: Into<Category>,
{
    todo!()
}

pub fn list_repositories_with_uncommitted_changes_in_category<T, U>(api: &Api<T>, category: U)
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
    U: Into<Category>,
{
    todo!()
}
