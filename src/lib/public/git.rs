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
