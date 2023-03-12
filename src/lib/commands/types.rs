use crate::iterators::SimpleCategoryDescriptionIterator;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SimpleRepositoryDescription {
    pub category: String,
    pub repository: String,
}

pub struct SimpleCategoryDescriptionV2 {
    pub category_directory: PathBuf,
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
        SimpleCategoryDescriptionIterator::new(self)
    }
}

impl SimpleRepositoryDescription {
    pub fn to_session_string(&self) -> String {
        format!("{}@{}", self.repository, self.category)
    }
}
