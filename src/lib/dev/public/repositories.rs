use crate::dev::{
    strategy::alias::{AliasStrategy, AliasStrategyError, ResolvesAlias, SupportsAlias},
    Api,
};

pub fn resolve_repository_alias<T, U>(
    api: &Api<T>,
    value: U,
) -> Result<U::Resolved, AliasStrategyError>
where
    T: SupportsAlias,
    U: ResolvesAlias,
{
    api.get_alias_strategy().resolve_alias(value)
}
