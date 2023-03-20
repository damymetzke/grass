use crate::{repository::repository_has_changes, types::SimpleCategoryDescription};

impl SimpleCategoryDescription {
    pub fn uncommitted_changes_only(mut self) -> Self {
        self.repositories.retain(|repository| {
            repository_has_changes(self.category_directory.join(&repository.repository))
        });
        self
    }

    pub fn no_uncommitted_changes(mut self) -> Self {
        self.repositories.retain(|repository| {
            !repository_has_changes(self.category_directory.join(&repository.repository))
        });
        self
    }
}
