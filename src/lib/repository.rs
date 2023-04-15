use std::path::Path;

use git2::Repository;

pub fn repository_has_changes<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    // TODO: Add error handling
    // TODO: Be more specific on errors
    let repository = Repository::init(path).ok();
    let repository = if let Some(repository) = repository {
        repository
    } else {
        return false;
    };
    let head = repository.head().ok().map(|head| head.peel_to_tree().ok());
    let head = if let Some(Some(head)) = head {
        head
    } else {
        return false;
    };

    let result = !matches!(
        repository
            .diff_tree_to_workdir(Some(&head), None,)
            .unwrap()
            .stats()
            .unwrap()
            .files_changed(),
        0
    );

    // Clippy gives a false positive here.
    // Inlining this return won't compile due to borrow rules.
    #[allow(clippy::let_and_return)]
    result
}
