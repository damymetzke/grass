use crate::{
    config,
    dev::strategy::{
        api::{ApiStrategy, LocalApiStrategy},
        git::{GitStrategy, GitStrategyError},
    },
};

pub struct RepositoryLocation {
    pub category: String,
    pub repository: String,
}

pub struct Api<T>
where
    T: ApiStrategy,
{
    strategy: T,
}

impl<T, U> From<(T, U)> for RepositoryLocation
where
    T: Into<String>,
    U: Into<String>,
{
    fn from((category, repository): (T, U)) -> Self {
        Self {
            category: category.into(),
            repository: repository.into(),
        }
    }
}

impl<'a> Api<LocalApiStrategy<'a>> {
}

impl<T> Api<T>
where
    T: ApiStrategy,
{
    pub fn as_local<U>(closure: U)
    where
        U: Fn(Api<LocalApiStrategy<'_>>),
    {
        // TODO: Set up error handling
        let config = config::load_user_config().unwrap();
        let strategy = LocalApiStrategy::new(&config.grass);
        closure(Api { strategy });
    }

    pub fn clean_repository<U>(&self, repository: U) -> Result<(), GitStrategyError>
    where
        U: Into<RepositoryLocation>,
    {
        self.strategy.get_git_strategy().clean(repository)
    }
}
