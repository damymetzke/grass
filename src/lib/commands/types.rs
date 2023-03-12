use crate::{config::RootConfig, iterators::SimpleCategoryDescriptionIterator, util};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SimpleRepositoryDescription {
    pub category: String,
    pub repository: String,
}

pub struct SimpleCategoryDescriptionV2 {
    category_directory: PathBuf,
    pub category: String,
    pub repositories: Vec<SimpleRepositoryDescription>,
}

#[deprecated = "Use SimpleCategoryDescriptionV2 instead"]
pub struct SimpleCategoryDescription {
    pub category: String,
    pub repositories: Vec<String>,
}

impl IntoIterator for SimpleCategoryDescriptionV2 {
    type Item = PathBuf;

    type IntoIter = SimpleCategoryDescriptionIterator;

    fn into_iter(self) -> Self::IntoIter {
        SimpleCategoryDescriptionIterator::new(self.repositories, self.category_directory)
    }
}

impl SimpleCategoryDescriptionV2 {
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
}

impl SimpleRepositoryDescription {
    pub fn to_session_string(&self) -> String {
        format!("{}@{}", self.repository, self.category)
    }
}
