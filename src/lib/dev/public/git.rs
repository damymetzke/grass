use crate::dev::{
    public::strategy::Api,
    strategy::{
        alias::{AliasStrategy, SupportsAlias},
        git::{GitStrategy, GitStrategyError, SupportsGit},
    },
};

use super::{api::RepositoryLocation, strategy::AccessApi};

/// Clean a git repository, by removing untracked and ignored files
///
/// Common examples of such files are build artifacts, cached data, and downloaded packages.
///
/// # Example
///
/// ```rust
/// # use grass::dev::{Api, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: Api<_> = api;
/// // This will clean the "first" repository, under the "all_good" category
/// grass::dev::clean_repository(&api, ("all_good", "first")).unwrap();
/// ```
pub fn clean_repository<T, U>(api: &Api<T>, repository: U) -> Result<(), GitStrategyError>
where
    T: SupportsGit + SupportsAlias,
    U: Into<RepositoryLocation>,
{
    api.get_strategy().get_git_strategy().clean(
        api.get_strategy()
            .get_alias_strategy()
            .resolve_location_alias(repository)?,
    )
}

/// Clone a git repository from a remote.
///
/// The remote must be a valid git remote.
/// Currently only publically available remotes are supported.
///
/// # Example
///
/// ```rust
/// # use grass::dev::{Api, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: Api<_> = api;
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
    T: SupportsGit + SupportsAlias,
    U: Into<RepositoryLocation>,
    V: AsRef<str>,
{
    let api = api.get_strategy();
    api.get_git_strategy().clone(
        api.get_alias_strategy()
            .resolve_location_alias(repository)?,
        remote,
    )
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
/// # use grass::dev::{Api, use_mock_strategy};
/// # let api = use_mock_strategy();
/// let api: Api<_> = api;
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
    T: SupportsGit + SupportsAlias,
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
