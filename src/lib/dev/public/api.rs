use std::fmt::Display;

use crate::dev::strategy::alias::{Alias, ResolveAliasResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Category(pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RepositoryLocation {
    pub category: Category,
    pub repository: String,
}

impl From<String> for Category {
    fn from(value: String) -> Self {
        Category(value)
    }
}

impl From<&String> for Category {
    fn from(value: &String) -> Self {
        Category(value.clone())
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        Category(value.to_owned())
    }
}

impl From<Alias> for Category {
    fn from(value: Alias) -> Self {
        value.category
    }
}

impl From<ResolveAliasResult> for Category {
    fn from(value: ResolveAliasResult) -> Self {
        match value {
            ResolveAliasResult::Alias(Alias { category, .. }) => category,
            ResolveAliasResult::NoAlias(category) => Category(category),
        }
    }
}

impl AsRef<String> for Category {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T, U> From<(T, U)> for RepositoryLocation
where
    T: Into<Category>,
    U: Into<String>,
{
    fn from((category, repository): (T, U)) -> Self {
        Self {
            category: category.into(),
            repository: repository.into(),
        }
    }
}

impl<T, U> From<&(T, U)> for RepositoryLocation
where
    T: Into<Category> + Clone,
    U: Into<String> + Clone,
{
    fn from((category, repository): &(T, U)) -> Self {
        Self {
            category: category.clone().into(),
            repository: repository.clone().into(),
        }
    }
}

impl RepositoryLocation {
    pub fn new<T, U>(category: T, repository: U) -> Self
    where
        T: Into<Category>,
        U: Into<String>,
    {
        RepositoryLocation {
            category: category.into(),
            repository: repository.into(),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
