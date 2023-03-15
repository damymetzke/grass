use std::path::{Path, PathBuf};

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

pub fn get_all_repository_changes<T, U>(items: T) -> impl Iterator<Item = (PathBuf, bool)>
where
    T: IntoIterator<Item = U>,
    U: Into<PathBuf>,
{
    items.into_iter().map(|path| {
        let path: PathBuf = path.into();

        (path.clone(), repository_has_changes(&path))
    })
}
