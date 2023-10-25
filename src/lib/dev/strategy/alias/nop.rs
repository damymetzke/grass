use super::{AliasStrategy, Result};

/// Implementation for `AliasStrategy`[^strategy], which does nothing.
///
/// Each method returns as if no aliases are defined.
///
/// [^strategy]: [crate::dev::strategy::alias::AliasStrategy]
pub struct NopAliasStrategy;

impl AliasStrategy for NopAliasStrategy {
    fn list_all_aliases<T>(&self) -> Result<T>
    where
        T: FromIterator<super::Alias>,
    {
        [].into_iter().collect()
    }

    fn list_aliases_for_category<T, U>(&self, _category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<super::Alias>,
    {
        [].into_iter().collect()
    }

    fn resolve_alias<T: super::ResolvesAlias>(&self, input: T) -> Result<T::Resolved> {
        input.resolve_alias(|value| Ok(Box::from(value)))
    }
}
