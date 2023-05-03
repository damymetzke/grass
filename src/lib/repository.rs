use std::path::Path;

use git2::{Repository, Status};

pub enum RepositoryChangeStatus {
    UpToDate,
    NoRepository,
    UncommittedChanges(usize),
}

pub fn get_repository_changes<T>(path: T) -> RepositoryChangeStatus
where
    T: AsRef<Path>,
{
    // TODO: Add error handling
    // TODO: Be more specific on errors
    let repository = Repository::open(path.as_ref()).ok();
    let repository = if let Some(repository) = repository {
        repository
    } else {
        return RepositoryChangeStatus::NoRepository;
    };

    let statuses = repository.statuses(None).ok();
    let statuses = if let Some(statuses) = statuses {
        statuses
    } else {
        return RepositoryChangeStatus::NoRepository;
    };

    match statuses
        .iter()
        .filter(|status| !matches!(status.status(), Status::IGNORED))
        .count()
    {
        0 => RepositoryChangeStatus::UpToDate,
        num_changes => RepositoryChangeStatus::UncommittedChanges(num_changes),
    }
}

pub fn repository_has_changes<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    !matches!(
        get_repository_changes(path),
        RepositoryChangeStatus::UpToDate
    )
}
