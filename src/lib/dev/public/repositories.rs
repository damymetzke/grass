use crate::dev::{
    strategy::alias::{
        AliasStrategy, AliasStrategyError, ResolveAliasResult, ResolvesAlias, SupportsAlias,
    },
    Api, RepositoryLocation,
};

use super::strategy::AccessApi;

#[deprecated = "use resolve_reposiotry_alias instead"]
pub fn resolve_repository_alias_from_str<T, U>(
    api: &Api<T>,
    value: U,
) -> Result<ResolveAliasResult, AliasStrategyError>
where
    T: SupportsAlias,
    U: AsRef<str>,
{
    api.get_strategy()
        .get_alias_strategy()
        .resolve_alias_old(value)
}

#[deprecated = "use resolve_reposiotry_alias instead"]
pub fn resolve_repository_alias_from_location<T, U>(
    api: &Api<T>,
    value: U,
) -> Result<RepositoryLocation, AliasStrategyError>
where
    T: SupportsAlias,
    U: Into<RepositoryLocation>,
{
    api.get_strategy()
        .get_alias_strategy()
        .resolve_location_alias(value)
}

pub fn resolve_repository_alias<T, U>(
    api: &Api<T>,
    value: U,
) -> Result<U::Resolved, AliasStrategyError>
where
    T: SupportsAlias,
    U: ResolvesAlias,
{
    api.get_strategy().get_alias_strategy().resolve_alias(value)
}
