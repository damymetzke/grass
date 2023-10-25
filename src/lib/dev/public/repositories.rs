use crate::dev::{
    strategy::alias::{AliasStrategy, AliasStrategyError, ResolvesAlias, SupportsAlias},
    Api,
};

use super::strategy::AccessApi;

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
