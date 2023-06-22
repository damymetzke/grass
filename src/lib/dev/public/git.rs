use crate::dev::{
    strategy::{
        api::ApiStrategy,
        git::{GitStrategy, GitStrategyError, RepositoryChangeStatus},
    },
    public::strategy::Api,
};

use super::{api::RepositoryLocation, strategy::AccessApi};

/// Clean a git repository, by removing untracked and ignored files
///
/// Common examples of such files are build artifacts, cached data, and downloaded packages.
///
/// # Example
///
/// ```rust
/// # use grass::dev::{ApiV2, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: ApiV2<_> = api;
/// // This will clean the "first" repository, under the "all_good" category
/// grass::dev::clean_repository(&api, ("all_good", "first")).unwrap();
/// ```
pub fn clean_repository<T, U>(api: &Api<T>, repository: U) -> Result<(), GitStrategyError>
where
    T: ApiStrategy,
    U: Into<RepositoryLocation>,
{
    api.get_strategy().get_git_strategy().clean(repository)
}

/// Clone a git repository from a remote.
///
/// The remote must be a valid git remote.
/// Currently only publically available remotes are supported.
///
/// # Example
///
/// ```rust
/// # use grass::dev::{ApiV2, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: ApiV2<_> = api;
/// // This will clone from the remote 'good_remote'.
/// // This will be cloned to the category 'all_good', with the repository name 'new_repository'.
/// grass::dev::clone_repository(&api, ("all_good", "new_repository"), "good_remote").unwrap();
/// ```
///
///
/// # Future
/// - TODO: Add a way to authenticate remotes
pub fn clone_repository<T, U, V>(
    api: &Api<T>,
    repository: U,
    remote: V,
) -> Result<(), GitStrategyError>
where
    T: ApiStrategy,
    U: Into<RepositoryLocation>,
    V: AsRef<str>,
{
    api.get_strategy()
        .get_git_strategy()
        .clone(repository, remote)
}

/// Clone a git repository from a remote, with an default generated name.
///
/// Has the same behavior as [crate::dev::clone_repository], except for the following difference:
///
/// - The repository name will be generated from the remote name.
///
/// So the remote 'https://example.com/foo.git', will have the repository name 'foo'.
///
/// # Example
///
/// ```rust
/// # use grass::dev::{ApiV2, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: ApiV2<_> = api;
/// // This will clone from the remote 'good_remote'.
/// // This will be cloned to the category 'all_good', with the repository name 'good_remote'.
/// grass::dev::clone_repository_default(&api, "all_good", "good_remote").unwrap();
/// ```
pub fn clone_repository_default<T, U, V>(
    api: &Api<T>,
    category: U,
    remote: V,
) -> Result<(), GitStrategyError>
where
    T: ApiStrategy,
    U: AsRef<str>,
    V: AsRef<str>,
{
    let repository = &remote.as_ref().to_string();
    let repository = repository
        .split('/')
        .last()
        .unwrap_or("repository")
        .trim_end_matches(".git");

    clone_repository(api, (category.as_ref(), repository), remote)
}

/// Get the change status of a specific repository
///
/// # Example
///
/// ```rust
/// # use grass::dev::{ApiV2, use_mock_strategy};
/// # use grass::dev::strategy::git::RepositoryChangeStatus;
/// # let api = use_mock_strategy();
/// let api: ApiV2<_> = api;
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
///     RepositoryChangeStatus::FilesChanged { num_changes: 9 }
/// );
/// ```
pub fn get_repository_change_status<T, U>(
    api: &Api<T>,
    repository: U,
) -> Result<RepositoryChangeStatus, GitStrategyError>
where
    T: ApiStrategy,
    U: Into<RepositoryLocation>,
{
    api.get_strategy()
        .get_git_strategy()
        .get_changes(repository)
}
