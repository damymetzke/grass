use std::{fmt::Display, path::Path};

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

impl Display for RepositoryChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryChangeStatus::UpToDate => write!(f, "No changes"),
            RepositoryChangeStatus::NoRepository => write!(f, "No repository"),
            RepositoryChangeStatus::UncommittedChanges(n) => write!(f, "{} uncommitted changes", n),
        }
    }
}
