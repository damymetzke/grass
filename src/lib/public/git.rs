use crate::dev::{
    strategy::{
        api::ApiStrategy,
        git::{GitStrategy, GitStrategyError},
    },
    Api,
};

use super::api::RepositoryLocation;

/// Clean a git repository, by removing untracked and ignored files
///
/// Common examples of such files are build artifacts, cached data, and downloaded packages.
///
/// # Example
///
/// ```rust
/// # use grass::dev::Api;
/// # Api::with_mock_strategy(|api|{
/// let api: Api<_> = api;
/// // This will clean the "first" repository, under the "all_good" category
/// grass::dev::clean_repository(&api, ("all_good", "first")).unwrap();
/// # });
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
/// # use grass::dev::Api;
/// # Api::with_mock_strategy(|api|{
/// let api: Api<_> = api;
/// // This will clone from the remote 'good_remote'.
/// // This will be cloned to the category 'all_good', with the repository name 'new_repository'.
/// grass::dev::clone_repository(&api, ("all_good", "new_repository"), "good_remote").unwrap();
/// # });
/// ```
///
///
/// # Future
/// - TODO: Add a way to authenticate remotes
pub fn clone_repository<T, U, V>(api: &Api<T>, repository: U, remote: V) -> Result<(), GitStrategyError>
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
/// # use grass::dev::Api;
/// # Api::with_mock_strategy(|api|{
/// let api: Api<_> = api;
/// // This will clone from the remote 'good_remote'.
/// // This will be cloned to the category 'all_good', with the repository name 'good_remote'.
/// grass::dev::clone_repository_default(&api, "all_good", "good_remote").unwrap();
/// # });
/// ```
pub fn clone_repository_default<T, U, V>(api: &Api<T>, category: U, remote: V) -> Result<(), GitStrategyError>
where
    T: ApiStrategy,
    U: AsRef<str>,
    V: AsRef<str>,
{
    api.get_strategy()
        .get_git_strategy()
        .clone_default(category, remote)
}
