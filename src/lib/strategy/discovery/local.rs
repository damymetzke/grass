use crate::{config::GrassConfig, public::api::RepositoryLocation};

use super::{DiscoveryExistsResult, DiscoveryStrategy, Result};

pub struct LocalDiscoveryStrategy<'a> {
    config: &'a GrassConfig,
}

impl<'a> LocalDiscoveryStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalDiscoveryStrategy { config }
    }
}

impl<'a> DiscoveryStrategy for LocalDiscoveryStrategy<'a> {
    fn check_repository_exists<T>(&self, repository: T) -> Result<DiscoveryExistsResult>
    where
        T: Into<RepositoryLocation>,
    {
        let RepositoryLocation {
            category,
            repository,
        } = repository.into();

        let category_dir = match self.config.get_from_category_or_alias(category) {
            Some(category) => self.config.base_dir.join(&category.name),
            None => return Ok(DiscoveryExistsResult::CategoryNotFound),
        };

        let repository_dir = category_dir.join(repository);

        match repository_dir.is_dir() {
            true => Ok(DiscoveryExistsResult::Exists),
            false => Ok(DiscoveryExistsResult::RepositoryNotFound),
        }
    }

    fn check_category_exists<T>(&self, category: T) -> Result<DiscoveryExistsResult>
    where
        T: AsRef<str>,
    {
        match self.config.get_from_category_or_alias(category) {
            Some(_) => Ok(DiscoveryExistsResult::Exists),
            None => Ok(DiscoveryExistsResult::CategoryNotFound),
        }
    }

    fn list_repositories_in_category<T, U>(&self, _category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<RepositoryLocation>,
    {
        todo!()
    }

    fn list_categories<T>(&self) -> Result<T>
    where
        T: FromIterator<String>,
    {
        todo!()
    }
}
