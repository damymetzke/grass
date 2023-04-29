use std::path::PathBuf;

use crate::types::SimpleRepositoryDescription;

pub struct SimpleCategoryDescriptionIterator<'a> {
    collection: &'a Vec<SimpleRepositoryDescription>,
    category_directory: &'a PathBuf,
    index: usize,
}

impl<'a> SimpleCategoryDescriptionIterator<'a> {
    pub fn new(collection: &'a Vec<SimpleRepositoryDescription>, category_directory: &'a PathBuf) -> Self {
        SimpleCategoryDescriptionIterator {
            collection,
            category_directory,
            index: 0,
        }
    }
}

impl<'a> Iterator for SimpleCategoryDescriptionIterator<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.collection.get(self.index) {
            Some(repository) => Some(self.category_directory.join(&repository.repository)),
            None => None,
        }
    }
}
