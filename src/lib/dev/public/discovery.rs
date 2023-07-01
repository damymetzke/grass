use crate::dev::{
    strategy::{
        api::SupportsDiscovery,
        discovery::{DiscoveryStrategy, DiscoveryStrategyError},
    },
    Api, RepositoryLocation,
};

use super::strategy::AccessApi;

pub fn list_repositories_in_category<T, U, V>(
    api: &Api<T>,
    category: U,
) -> Result<V, DiscoveryStrategyError>
where
    T: SupportsDiscovery,
    U: AsRef<str>,
    V: FromIterator<RepositoryLocation>,
{
    let iterator = api
        .get_strategy()
        .get_discovery_strategy()
        .list_repositories_in_category(category)?;

    Ok(iterator
        .filter_map(|value| match value {
            Ok(value) => Some(value),
            Err(_) => None,
        })
        .collect())
}

pub fn list_repositories_in_category_with_errors<T, U, V>(
    api: &Api<T>,
    category: U,
) -> Result<V, DiscoveryStrategyError>
where
    T: SupportsDiscovery,
    U: AsRef<str>,
    V: FromIterator<Result<RepositoryLocation, DiscoveryStrategyError>>,
{
    Ok(api
        .get_strategy()
        .get_discovery_strategy()
        .list_repositories_in_category(category)?
        .collect())
}

pub fn list_all_repositories<T, U>(api: &Api<T>) -> Result<U, DiscoveryStrategyError>
where
    T: SupportsDiscovery,
    U: FromIterator<RepositoryLocation>,
{
    let categories: Vec<_> = list_categories(api)?;
    Ok(categories
        .iter()
        .filter_map(|category| list_repositories_in_category::<_, _, Vec<_>>(api, category).ok())
        .flatten()
        .collect())
}

pub fn list_categories<T: SupportsDiscovery, U: FromIterator<String>>(
    api: &Api<T>,
) -> Result<U, DiscoveryStrategyError> {
    api.get_strategy()
        .get_discovery_strategy()
        .list_categories()
}
