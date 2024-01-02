use crate::dev::{
    strategy::{
        alias::{AliasStrategy, SupportsAlias},
        discovery::{
            DiscoveryExists, DiscoveryStrategy, DiscoveryStrategyError, SupportsDiscovery,
        },
    },
    Api, Category, RepositoryLocation,
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
    let category: Category = api
        .get_strategy()
        .get_alias_strategy()
        .resolve_alias(category.as_ref())?
        .into();
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
    let category: Category = api
        .get_strategy()
        .get_alias_strategy()
        .resolve_alias(category.as_ref())?
        .into();

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

/// Returns whether or not a repository exists
///
/// # Todo:
///
/// - [ ] Return a generic crate wide error
///       (<https://github.com/damymetzke/grass/issues/2>)
///
/// # Example
///
/// ```rust
/// # use grass::dev::{
/// #     self,
/// #     strategy::{
/// #         alias::SupportsAlias,
/// #         api::MockApiStrategy,
/// #         discovery::{DiscoveryExists, SupportsDiscovery},
/// #     },
/// #     Api,
/// # };
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_public<T: SupportsDiscovery + SupportsAlias>(api: &Api<T>) {
///     assert_eq!(
///         dev::verify_repository_exists(api, ("all_good", "first")).unwrap(),
///         DiscoveryExists::Exists
///     )
/// }
///
/// test_public(&api)
/// ```
pub fn verify_repository_exists<T, U>(
    api: &Api<T>,
    repository: U,
) -> Result<DiscoveryExists, DiscoveryStrategyError>
where
    T: SupportsDiscovery + SupportsAlias,
    U: Into<RepositoryLocation>,
{
    let api = api.get_strategy();
    let result = api
        .get_discovery_strategy()
        .check_repository_exists(api.get_alias_strategy().resolve_alias(repository.into())?)?;

    Ok(result)
}

pub fn create_repository<T, U>(api: &Api<T>, location: U) -> Result<(), DiscoveryStrategyError>
where
    T: SupportsDiscovery + SupportsAlias,
    U: Into<RepositoryLocation>,
{
    let api = api.get_strategy();
    let alias = api.get_alias_strategy();
    let discovery = api.get_discovery_strategy();

    let location = alias.resolve_alias(location.into())?;
    discovery.create_repository(location)?;

    Ok(())
}

pub fn move_repository<T, U, V>(
    api: &Api<T>,
    old_location: U,
    new_location: V,
) -> Result<(), DiscoveryStrategyError>
where
    T: SupportsDiscovery + SupportsAlias,
    U: Into<RepositoryLocation>,
    V: Into<RepositoryLocation>,
{
    let alias = api.get_alias_strategy();
    let api = api.get_strategy();
    let discovery = api.get_discovery_strategy();

    let old_location = alias.resolve_alias(old_location.into())?;
    let new_location = alias.resolve_alias(new_location.into())?;
    discovery.move_repository(old_location, new_location)?;

    Ok(())
}
