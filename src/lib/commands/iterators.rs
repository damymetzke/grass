use std::path::PathBuf;

use crate::types::SimpleRepositoryDescription;

pub struct SimpleCategoryDescriptionIterator {
    collection: Vec<SimpleRepositoryDescription>,
    category_directory: PathBuf,
    index: usize,
}

impl SimpleCategoryDescriptionIterator {
    pub fn new(collection: Vec<SimpleRepositoryDescription>, category_directory: PathBuf) -> Self {
        SimpleCategoryDescriptionIterator {
            collection,
            category_directory,
            index: 0,
        }
    }
}

impl Iterator for SimpleCategoryDescriptionIterator {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.collection.get(self.index) {
            Some(repository) => Some(self.category_directory.join(&repository.repository)),
            None => None,
        }
    }
}
