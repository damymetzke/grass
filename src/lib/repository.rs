use std::path::Path;

use git2::Repository;

pub fn repository_has_changes<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    // TODO: Add error handling
    let repository = Repository::init(path).unwrap();
    let head = repository.head().unwrap().peel_to_tree().unwrap();
    let result = !matches!(repository
        .diff_tree_to_workdir(
            Some(&head),
            None,
        )
        .unwrap()
        .stats()
        .unwrap()
        .files_changed(), 0);

    result
}
