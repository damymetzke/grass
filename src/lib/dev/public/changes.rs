use crate::dev::{
    error::GrassError,
    strategy::{
        alias::{AliasStrategy, SupportsAlias},
        discovery::{DiscoveryStrategy, DiscoveryStrategyError, SupportsDiscovery},
        git::{
            GitStrategy, GitStrategyError, RepositoryChangeStatus, RepositoryChangeStatusWithError,
            SupportsGit,
        },
    },
    Api, Category, RepositoryLocation,
};

use super::strategy::AccessApi;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChangeStatusResult {
    pub location: Option<RepositoryLocation>,
    pub change_status: RepositoryChangeStatusWithError,
}

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

fn location_result_to_change_status_result<T: GitStrategy>(
    value: (Result<RepositoryLocation, DiscoveryStrategyError>, &T),
) -> ChangeStatusResult {
    let (location, git) = value;

    let location = match location {
        Ok(location) => location.clone(),
        Err(error) => {
            return ChangeStatusResult {
                location: None,
                change_status: error.into(),
            }
        }
    };
    ChangeStatusResult {
        location: Some(location.clone()),
        change_status: git.get_changes(location).into(),
    }
}

fn filter_away_up_to_date_repositories(value: &ChangeStatusResult) -> bool {
    !matches!(
        value,
        ChangeStatusResult {
            change_status: RepositoryChangeStatusWithError::UpToDate,
            ..
        }
    )
}

/// List all repositories, including their change status.
///
/// This will also list repositories which are up to date.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// #
/// # use grass::dev::{
/// #     self,
/// #     ChangeStatusResult,
/// #     strategy::{
/// #         api::MockApiStrategy,
/// #         discovery::SupportsDiscovery,
/// #         git::{RepositoryChangeStatusWithError, SupportsGit},
/// #     },
/// #     Api,
/// # };
/// #
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_api<T: SupportsGit + SupportsDiscovery>(api: &Api<T>) {
///     let repositories: HashSet<_> =
///         dev::list_repositories_with_change_status_next(api).unwrap();
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "first").into()),
///         change_status: RepositoryChangeStatusWithError::UpToDate,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "second").into()),
///         change_status: RepositoryChangeStatusWithError::NoRepository,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "third").into()),
///         change_status: RepositoryChangeStatusWithError::UncommittedChanges {
///             num_changes: 9
///         },
///     }));
/// }
///
/// test_api(&api)
/// ```
pub fn list_repositories_with_change_status<T, U>(api: &Api<T>) -> Result<U, GrassError>
where
    T: SupportsGit + SupportsDiscovery,
    U: FromIterator<ChangeStatusResult>,
{
    let api = api.get_strategy();
    let git = api.get_git_strategy();
    let discovery = api.get_discovery_strategy();

    let categories: Vec<_> = discovery.list_categories()?;
    Ok(categories
        .iter()
        .filter_map(|category| discovery.list_repositories_in_category(category).ok())
        .flat_map(|repositories| {
            repositories
                .map(|repository| (repository, git))
                .map(location_result_to_change_status_result)
        })
        .collect())
}

/// List all repositories with uncommitted changes, including their change status.
///
/// This won't list repositories that are up to date.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// #
/// # use grass::dev::{
/// #     self,
/// #     ChangeStatusResult,
/// #     strategy::{
/// #         api::MockApiStrategy,
/// #         discovery::SupportsDiscovery,
/// #         git::{RepositoryChangeStatusWithError, SupportsGit},
/// #     },
/// #     Api,
/// # };
/// #
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_api<T: SupportsGit + SupportsDiscovery>(api: &Api<T>) {
///     let repositories: HashSet<_> =
///         dev::list_repositories_with_uncommitted_changes(api).unwrap();
///
///     assert!(!repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "first").into()),
///         change_status: RepositoryChangeStatusWithError::UpToDate,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "second").into()),
///         change_status: RepositoryChangeStatusWithError::NoRepository,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "third").into()),
///         change_status: RepositoryChangeStatusWithError::UncommittedChanges {
///             num_changes: 9
///         },
///     }));
/// }
///
/// test_api(&api)
/// ```
pub fn list_repositories_with_uncommitted_changes<T, U>(api: &Api<T>) -> Result<U, GrassError>
where
    T: SupportsGit + SupportsDiscovery,
    U: FromIterator<ChangeStatusResult>,
{
    let api = api.get_strategy();
    let git = api.get_git_strategy();
    let discovery = api.get_discovery_strategy();

    let categories: Vec<_> = discovery.list_categories()?;
    Ok(categories
        .iter()
        .filter_map(|category| discovery.list_repositories_in_category(category).ok())
        .flat_map(|repositories| {
            repositories
                .map(|repository| (repository, git))
                .map(location_result_to_change_status_result)
                .filter(filter_away_up_to_date_repositories)
        })
        .collect())
}

/// List all repositories in a specified category, including their change status.
///
/// This will also list repositories which are up to date.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// #
/// # use grass::dev::{
/// #     self,
/// #     ChangeStatusResult,
/// #     strategy::{
/// #         alias::SupportsAlias,
/// #         api::MockApiStrategy,
/// #         discovery::SupportsDiscovery,
/// #         git::{RepositoryChangeStatusWithError, SupportsGit},
/// #     },
/// #     Api,
/// # };
/// #
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_api<T: SupportsGit + SupportsDiscovery + SupportsAlias>(api: &Api<T>) {
///     let repositories: HashSet<_> =
///         dev::list_repositories_with_change_status_in_category(api, "with_changes").unwrap();
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "second").into()),
///         change_status: RepositoryChangeStatusWithError::NoRepository,
///     }));
///
///     assert!(!repositories.contains(&ChangeStatusResult {
///         location: Some(("all_good", "second").into()),
///         change_status: RepositoryChangeStatusWithError::UncommittedChanges {
///             num_changes: 9
///         },
///     }));
/// }
///
/// test_api(&api)
/// ```
pub fn list_repositories_with_change_status_in_category<T, U, V>(
    api: &Api<T>,
    category: U,
) -> Result<V, GrassError>
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
    U: Into<Category>,
    V: FromIterator<ChangeStatusResult>,
{
    let api = api.get_strategy();
    let alias = api.get_alias_strategy();
    let git = api.get_git_strategy();
    let discovery = api.get_discovery_strategy();

    let category: Category = alias.resolve_alias_old(category.into())?.into();

    let repositories = discovery.list_repositories_in_category::<Category>(category)?;

    Ok(repositories
        .map(|repository| (repository, git))
        .map(location_result_to_change_status_result)
        .collect())
}

/// List all repositories with uncommitted changes, in a specific category, including their change status.
///
/// This won't list repositories that are up to date.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// #
/// # use grass::dev::{
/// #     self,
/// #     ChangeStatusResult,
/// #     strategy::{
/// #         alias::SupportsAlias,
/// #         api::MockApiStrategy,
/// #         discovery::SupportsDiscovery,
/// #         git::{RepositoryChangeStatusWithError, SupportsGit},
/// #     },
/// #     Api,
/// # };
/// #
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_api<T: SupportsGit + SupportsDiscovery + SupportsAlias>(api: &Api<T>) {
///     let repositories: HashSet<_> =
///         dev::list_repositories_with_uncommitted_changes_in_category(api, "with_changes").unwrap();
///
///     assert!(!repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "first").into()),
///         change_status: RepositoryChangeStatusWithError::UpToDate,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "second").into()),
///         change_status: RepositoryChangeStatusWithError::NoRepository,
///     }));
///
///     assert!(repositories.contains(&ChangeStatusResult {
///         location: Some(("with_changes", "third").into()),
///         change_status: RepositoryChangeStatusWithError::UncommittedChanges {
///             num_changes: 9
///         },
///     }));
/// }
///
/// test_api(&api)
/// ```
pub fn list_repositories_with_uncommitted_changes_in_category<T, U, V>(
    api: &Api<T>,
    category: U,
) -> Result<V, GrassError>
where
    T: SupportsGit + SupportsAlias + SupportsDiscovery,
    U: Into<Category>,
    V: FromIterator<ChangeStatusResult>,
{
    let api = api.get_strategy();
    let alias = api.get_alias_strategy();
    let git = api.get_git_strategy();
    let discovery = api.get_discovery_strategy();

    let category: Category = alias.resolve_alias_old(category.into())?.into();

    let repositories = discovery.list_repositories_in_category::<Category>(category)?;

    Ok(repositories
        .map(|repository| (repository, git))
        .map(location_result_to_change_status_result)
        .filter(filter_away_up_to_date_repositories)
        .collect())
}
