use crate::{config::RootConfig, iterators::SimpleCategoryDescriptionIterator, util};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SimpleRepositoryDescription {
    pub category: String,
    pub repository: String,
}

pub struct SimpleCategoryDescription {
    category_directory: PathBuf,
    pub category: String,
    pub repositories: Vec<SimpleRepositoryDescription>,
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

    pub fn iter_paths(&self) -> SimpleCategoryDescriptionIterator {
        SimpleCategoryDescriptionIterator::new(&self.repositories, &self.category_directory)
    }
}

impl SimpleRepositoryDescription {
    pub fn to_session_string(&self) -> String {
        format!("{}@{}", self.repository, self.category)
    }
}
