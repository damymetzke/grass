use crate::{
    config,
    dev::strategy::api::{ApiStrategy, LocalApiStrategy},
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

impl<'a> Api<LocalApiStrategy<'a>> {}

impl<T> Api<T>
where
    T: ApiStrategy,
{
    /// Create an API intance using the 'local' strategy
    ///
    /// The 'local' strategy applies to the local system.
    /// This is considered the main focus of the crate.
    pub fn with_local_strategy<U>(closure: U)
    where
        U: Fn(Api<LocalApiStrategy<'_>>),
    {
        // TODO: Set up error handling
        let config = config::load_user_config().unwrap();
        let strategy = LocalApiStrategy::new(&config.grass);
        closure(Api { strategy });
    }

    /// Reference to the internal strategy object
    ///
    /// Typically shouldn't be accessed directly.
    /// The intended use of the API is to use the other API methods.
    pub fn get_strategy(&self) -> &T {
        &self.strategy
    }
}
