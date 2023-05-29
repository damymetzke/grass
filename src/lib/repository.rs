use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use git2::{Repository, Status};

pub enum RepositoryChangeStatus {
    UpToDate,
    NoRepository,
    UncommittedChanges(usize),
}

pub enum GetRepositoryChangersError {
    CannotOpenRepository { path: PathBuf, reason: String },
    CannotReadStatus { path: PathBuf, reason: String },
}

pub fn get_repository_changes<T>(
    path: T,
) -> Result<RepositoryChangeStatus, GetRepositoryChangersError>
where
    T: AsRef<Path>,
{
    let repository = Repository::open(path.as_ref());
    let repository = match repository {
        Ok(repository) => repository,
        Err(error) => match error.code() {
            git2::ErrorCode::NotFound => todo!(),
            _ => {
                return Err(GetRepositoryChangersError::CannotOpenRepository {
                    path: path.as_ref().into(),
                    reason: error.message().to_string(),
                })
            }
        },
    };

    let statuses = repository.statuses(None);
    let statuses = match statuses {
        Ok(statuses) => statuses,
        Err(error) => {
            return Err(GetRepositoryChangersError::CannotReadStatus {
                path: path.as_ref().into(),
                reason: error.message().to_string(),
            })
        }
    };
    match statuses
        .iter()
        .filter(|status| !matches!(status.status(), Status::IGNORED))
        .count()
    {
        0 => Ok(RepositoryChangeStatus::UpToDate),
        num_changes => Ok(RepositoryChangeStatus::UncommittedChanges(num_changes)),
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
