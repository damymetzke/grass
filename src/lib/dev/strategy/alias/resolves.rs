use std::{rc::Rc, sync::Arc};

use crate::dev::{Category, RepositoryLocation};

use super::AliasStrategyError;

pub trait ResolvesAlias {
    type Resolved;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError>;
}

impl ResolvesAlias for String {
    type Resolved = String;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        Ok(String::from(resolver(self.as_str())?))
    }
}

impl ResolvesAlias for Box<str> {
    type Resolved = Box<str>;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        resolver(self.as_ref())
    }
}

impl ResolvesAlias for Rc<str> {
    type Resolved = Rc<str>;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        Ok(Rc::from(resolver(self.as_ref())?))
    }
}

impl ResolvesAlias for Arc<str> {
    type Resolved = Arc<str>;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        Ok(Arc::from(resolver(self.as_ref())?))
    }
}

impl ResolvesAlias for str {
    type Resolved = Box<str>;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        resolver(self)
    }
}

impl ResolvesAlias for &str {
    type Resolved = Box<str>;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        resolver(self)
    }
}


impl ResolvesAlias for RepositoryLocation {
    type Resolved = RepositoryLocation;

    fn resolve_alias<F: FnOnce(&str) -> Result<Box<str>, AliasStrategyError>>(
        &self,
        resolver: F,
    ) -> Result<Self::Resolved, AliasStrategyError> {
        Ok(RepositoryLocation {
            category: Category::from(resolver(self.category.as_ref())?.as_ref()),
            repository: self.repository.clone(),
        })
    }
}
