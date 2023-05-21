use crate::{config::RootConfig, util};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SimpleRepositoryDescription {
    pub category: String,
    pub repository: String,
}

#[derive(Debug, Clone)]
pub struct SimpleCategoryDescription {
    pub category_directory: PathBuf,
    pub category: String,
    pub repositories: Vec<SimpleRepositoryDescription>,
}

#[derive(Debug, Clone)]
pub struct FilteredCategoryDescription<T, U>
where
    T: Iterator<Item = (SimpleRepositoryDescription, U)>,
{
    pub category_directory: PathBuf,
    pub category: String,
    pub repository_iterator: T,
}

impl IntoIterator for SimpleCategoryDescription {
    type Item = SimpleRepositoryDescription;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.repositories.into_iter()
    }
}

impl SimpleCategoryDescription {
    pub fn new<T, U>(config: &RootConfig, category: T, repositories: U) -> Self
    where
        T: Into<String>,
        U: IntoIterator<Item = SimpleRepositoryDescription>,
    {
        // TODO: Do error checking
        let category = category.into();
        Self {
            category_directory: util::get_base_directory(config).unwrap().join(&category),
            category,
            repositories: repositories.into_iter().collect(),
        }
    }

    pub fn iter_paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.repositories
            .iter()
            .map(|repository| self.category_directory.join(&repository.repository))
    }
}

impl<T, U> FilteredCategoryDescription<T, U>
where
    T: Iterator<Item = (SimpleRepositoryDescription, U)>,
{
    pub fn new<V>(config: &RootConfig, category: V, repository_iterator: T) -> Self
    where
        V: Into<String>,
    {
        // TODO: Do error checking
        let category = category.into();
        Self {
            category_directory: util::get_base_directory(config).unwrap().join(&category),
            category,
            repository_iterator,
        }
    }

    pub fn iter_paths(self) -> impl Iterator<Item = PathBuf> {
        self.repository_iterator
            .map(move |(repository, _)| self.category_directory.join(repository.repository))
    }
}

impl SimpleRepositoryDescription {
    pub fn to_session_string(&self) -> String {
        format!("{}@{}", self.repository, self.category)
    }
}
