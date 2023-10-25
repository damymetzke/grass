use crate::dev::config::GrassConfig;

use super::{Alias, AliasStrategy, AliasStrategyError, Result};

pub struct LocalAliasStrategy<'a> {
    config: &'a GrassConfig,
}

impl<'a> AliasStrategy for LocalAliasStrategy<'a> {
    fn list_all_aliases<T>(&self) -> Result<T>
    where
        T: FromIterator<super::Alias>,
    {
        let result = self
            .config
            .category
            .iter()
            .flat_map(|(_, category)| {
                let category = category.borrow();
                let result: Vec<_> = category
                    .alias
                    .iter()
                    .map(|alias| Alias {
                        alias: alias.into(),
                        category: category.name.clone().into(),
                    })
                    .collect();
                result
            })
            .collect();

        Ok(result)
    }

    fn list_aliases_for_category<T, U>(&self, category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<super::Alias>,
    {
        let category = match self.config.category.get(category.as_ref()) {
            Some(category) => category,
            None => {
                return Err(AliasStrategyError::CategoryNotFound {
                    context: "When retrieving the category from the configuration".into(),
                    reason: "Category doesn't exist".into(),
                })
            }
        };

        let category = category.borrow();

        let result = category
            .alias
            .iter()
            .map(|alias| Alias {
                alias: alias.into(),
                category: category.name.clone().into(),
            })
            .collect();

        Ok(result)
    }

    fn resolve_alias<T: super::ResolvesAlias>(&self, input: T) -> Result<T::Resolved> {
        input.resolve_alias(|input| match self.config.aliases.get(input) {
            Some(alias) => Ok(Box::from((*alias.as_ref()).borrow().name.as_ref())),
            None => Ok(Box::from(input)),
        })
    }
}

impl<'a> LocalAliasStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalAliasStrategy { config }
    }
}
