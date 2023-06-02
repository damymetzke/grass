use crate::dev::{strategy::{api::ApiStrategy, git::{GitStrategyError, GitStrategy}}, Api};

use super::api::RepositoryLocation;

impl<T> Api<T>
where
    T: ApiStrategy,
{
    /// Clean a git repository
    ///
    /// This will remove any files which are both untracked and ignored.
    /// Typically these include:
    ///
    /// - Build artifacts
    /// - Cached data
    /// - Downloaded packages
    pub fn clean_repository<U>(&self, repository: U) -> Result<(), GitStrategyError>
    where
        U: Into<RepositoryLocation>,
    {
        self.get_strategy().get_git_strategy().clean(repository)
    }
}
