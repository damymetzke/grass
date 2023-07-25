use crate::dev::{
    strategy::{discovery::{DiscoveryStrategy, DiscoveryStrategyError, SupportsDiscovery}, alias::{SupportsAlias, AliasStrategy}},
    Api, RepositoryLocation, Category,
};

use super::strategy::AccessApi;

pub fn list_repositories_in_category<T, U, V>(
    api: &Api<T>,
    category: U,
) -> Result<V, DiscoveryStrategyError>
where
    T: SupportsDiscovery + SupportsAlias,
    U: AsRef<str>,
    V: FromIterator<RepositoryLocation>,
{

    let category: Category = api.get_strategy().get_alias_strategy().resolve_alias(category)?.into();
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
    T: SupportsDiscovery + SupportsAlias,
    U: AsRef<str>,
    V: FromIterator<Result<RepositoryLocation, DiscoveryStrategyError>>,
{
    let category: Category = api.get_strategy().get_alias_strategy().resolve_alias(category)?.into();

    Ok(api
        .get_strategy()
        .get_discovery_strategy()
        .list_repositories_in_category(category)?
        .collect())
}

/// List all repositories detected by the API
///
/// # Todo:
///
/// - [ ] Change it so it doesn't require `SupportsAlias`
pub fn list_all_repositories<T, U>(api: &Api<T>) -> Result<U, DiscoveryStrategyError>
where
    T: SupportsDiscovery + SupportsAlias,
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
