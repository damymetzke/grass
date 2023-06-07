use crate::dev::{
    strategy::{
        api::ApiStrategy,
        git::{GitStrategy, GitStrategyError},
    },
    Api,
};

use super::api::RepositoryLocation;

impl<T> Api<T>
where
    T: ApiStrategy,
{
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
    /// api.clean_repository(("all_good", "first")).unwrap();
    /// # });
    /// ```
    pub fn clean_repository<U>(&self, repository: U) -> Result<(), GitStrategyError>
    where
        U: Into<RepositoryLocation>,
    {
        self.get_strategy().get_git_strategy().clean(repository)
    }

    pub fn clone_repository<U, V>(&self, repository: U, remote: V) -> Result<(), GitStrategyError>
    where
        U: Into<RepositoryLocation>,
        V: AsRef<str>,
    {
        self.get_strategy()
            .get_git_strategy()
            .clone(repository, remote)
    }

    pub fn clone_repository_default<U, V>(
        &self,
        category: U,
        remote: V,
    ) -> Result<(), GitStrategyError>
    where
        U: AsRef<str>,
        V: AsRef<str>,
    {
        self.get_strategy()
            .get_git_strategy()
            .clone_default(category, remote)
    }
}
