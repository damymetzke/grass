use crate::dev::{
    strategy::alias::{AliasStrategy, AliasStrategyError, ResolveAliasResult, SupportsAlias},
    Api, RepositoryLocation,
};

use super::strategy::AccessApi;


pub fn resolve_repository_alias_from_str<T, U>(
    api: &Api<T>,
    value: U,
) -> Result<ResolveAliasResult, AliasStrategyError>
where
    T: SupportsAlias,
    U: AsRef<str>,
{
    api.get_strategy().get_alias_strategy().resolve_alias(value)
}

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
