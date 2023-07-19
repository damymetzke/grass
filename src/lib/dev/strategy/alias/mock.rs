use super::{Alias, AliasStrategy, AliasStrategyError, ResolveAliasResult};

/// Mocking implementation for `AliasStrategy`[^strategy].
///
/// # Todo:
///
/// - [ ] Write a new mocking structure and reference it here.
///
/// [^strategy]: [crate::dev::strategy::alias::AliasStrategy]
#[derive(Debug, Default)]
pub struct MockAliasStrategy;

impl AliasStrategy for MockAliasStrategy {
    fn list_all_aliases<T>(&self) -> super::Result<T>
    where
        T: FromIterator<super::Alias>,
    {
        Ok([
            ("allg", "all_good"),
            ("change", "with_changes"),
            ("err", "with_error"),
        ]
        .into_iter()
        .map(Into::<Alias>::into)
        .collect())
    }

    fn list_aliases_for_category<T, U>(&self, category: T) -> super::Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<super::Alias>,
    {
        let result = match category.as_ref() {
            "all_good" => [("allg", "all_good")],
            "with_changes" => [("change", "with_changes")],
            "with_error" => [("err", "with_error")],
            _ => {
                return Err(AliasStrategyError::CategoryNotFound {
                    context: "When mocking the API".into(),
                    reason: "Category isn't defined".into(),
                })
            }
        };

        Ok(result.into_iter().map(Into::<Alias>::into).collect())
    }

    fn resolve_alias<T>(&self, alias: T) -> super::Result<super::ResolveAliasResult>
    where
        T: AsRef<str>,
    {
        let result = match alias.as_ref() {
            "allg" => ("allg", "all_good"),
            "with_changes" => ("change", "with_changes"),
            "with_error" => ("err", "with_error"),
            _ => return Ok(ResolveAliasResult::NoAlias(alias.as_ref().into())),
        };

        Ok(super::ResolveAliasResult::Alias(result.into()))
    }
}
