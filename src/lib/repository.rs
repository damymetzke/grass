use std::path::Path;

use git2::{Repository, Status};

pub fn repository_has_changes<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    // TODO: Add error handling
    // TODO: Be more specific on errors
    let repository = Repository::open(path.as_ref()).ok();
    let repository = if let Some(repository) = repository {
        repository
    } else {
        return true;
    };

    let statuses = repository.statuses(None).ok();
    let statuses = if let Some(statuses) = statuses {
        statuses
    } else {
        return true;
    };

    statuses
        .iter()
        .filter(|status| !matches!(status.status(), Status::IGNORED))
        .count()
        > 0
}
