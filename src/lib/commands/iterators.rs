use std::path::PathBuf;

use crate::types::SimpleCategoryDescriptionV2;

pub struct SimpleCategoryDescriptionIterator {
    collection: SimpleCategoryDescriptionV2,
    index: usize,
}

impl SimpleCategoryDescriptionIterator {
    pub fn new(collection: SimpleCategoryDescriptionV2) -> Self {
        SimpleCategoryDescriptionIterator {
            collection,
            index: 0,
        }
    }
}

impl Iterator for SimpleCategoryDescriptionIterator {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.collection.repositories.get(self.index) {
            Some(repository) => Some(
                self.collection
                    .category_directory
                    .join(&repository.repository),
            ),
            None => None,
        }
    }
}
