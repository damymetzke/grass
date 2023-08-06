use git2::{build::RepoBuilder, ErrorCode, Repository, Status};
use itertools::Itertools;

use crate::dev::strategy::path::{PathStrategy, PathStrategyError};

use super::{GitStrategy, GitStrategyError, RepositoryChangeStatus, RepositoryLocation, Result};

pub struct LocalGitStrategy<'a, T: PathStrategy> {
    path_strategy: &'a T,
}

impl<'a, T: PathStrategy> LocalGitStrategy<'a, T> {
    pub fn new(path_strategy: &'a T) -> Self {
        Self { path_strategy }
    }
}

impl<'a, T: PathStrategy> GitStrategy for LocalGitStrategy<'a, T> {
    fn clean<U>(&self, repository: U) -> Result<()>
    where
        U: Into<RepositoryLocation>,
    {
        let repository_path = self.path_strategy.get_directory(repository)?;

        let repository = Repository::open(&repository_path)?;

        let statuses = repository.statuses(None)?;

        let results: Vec<_> = statuses
            .iter()
            .filter_map(|entry| match entry.path() {
                Some(file_path) if entry.status().is_ignored() => Some(file_path.to_owned()),
                _ => None,
            })
            .map(|file| {
                let path = repository_path.join(file);
                if path.is_dir() {
                    std::fs::remove_dir_all(path)
                } else {
                    std::fs::remove_file(path)
                }
            })
            .filter_map(std::result::Result::err)
            .map(|error| error.to_string())
            .collect();

        match results.len() {
            0 => Ok(()),
            _ => Err(GitStrategyError::FileSystemError {
                message: "Something went wrong when trying to clean a repository".into(),
                reason: results.iter().map(ToString::to_string).join("\n"),
                reasons: results,
            }),
        }
    }

    fn clone<U, V>(&self, repository: U, remote: V) -> Result<()>
    where
        U: Into<RepositoryLocation>,
        V: AsRef<str>,
    {
        let repo_path = self.path_strategy.get_directory(repository)?;

        RepoBuilder::new().clone(remote.as_ref(), &repo_path)?;

        Ok(())
    }

    fn get_changes<U>(&self, repository: U) -> Result<RepositoryChangeStatus>
    where
        U: Into<RepositoryLocation>,
    {
        let repository_path = self.path_strategy.get_directory(repository)?;

        let repository = match Repository::open(repository_path) {
            Ok(repository) => Ok(repository),
            Err(error) => {
                if error.code() == ErrorCode::NotFound {
                    return Ok(RepositoryChangeStatus::UpToDate);
                }

                Err(error)
            }
        };

        let repository = repository.map_err(|error| {
            GitStrategyError::from(error).with_message("There was a problem opening the repository")
        })?;

        let statuses = repository.statuses(None).map_err(|error| {
            GitStrategyError::from(error)
                .with_message("There was a problem retrieving the repository status")
        })?;

        let changes: Vec<_> = statuses
            .iter()
            .filter(|status| status.status().contains(Status::IGNORED))
            .collect();

        if changes.is_empty() {
            return Ok(RepositoryChangeStatus::UpToDate);
        }

        Ok(RepositoryChangeStatus::UncommittedChanges {
            num_changes: changes.len(),
        })
    }
}

impl From<PathStrategyError> for GitStrategyError {
    fn from(value: PathStrategyError) -> Self {
        match value {
            PathStrategyError::RepositoryNotFound { context, reason } => {
                GitStrategyError::RepositoryNotFound {
                    message: context,
                    reason,
                }
            }
            PathStrategyError::FileDoesNotExist { context, reason } => {
                GitStrategyError::FileSystemError {
                    message: context,
                    reason,
                    reasons: Vec::new(),
                }
            }
            PathStrategyError::Unknown { context, reason } => GitStrategyError::UnknownError {
                message: context,
                reason,
            },
        }
    }
}

impl From<git2::Error> for GitStrategyError {
    fn from(value: git2::Error) -> Self {
        let reason = value.message().to_string();
        match value.code() {
            git2::ErrorCode::NotFound => GitStrategyError::RepositoryNotFound {
                message: "There was a problem running an unspecified git2 command".into(),
                reason,
            },

            git2::ErrorCode::Exists => GitStrategyError::RepositryExists {
                message: "There was a problem running an unspecified git2 command".into(),
                reason,
            },

            git2::ErrorCode::Ambiguous
            | git2::ErrorCode::BufSize
            | git2::ErrorCode::User
            | git2::ErrorCode::BareRepo
            | git2::ErrorCode::UnbornBranch
            | git2::ErrorCode::Unmerged
            | git2::ErrorCode::NotFastForward
            | git2::ErrorCode::InvalidSpec
            | git2::ErrorCode::Conflict
            | git2::ErrorCode::Locked
            | git2::ErrorCode::Modified
            | git2::ErrorCode::Applied
            | git2::ErrorCode::Peel
            | git2::ErrorCode::Eof
            | git2::ErrorCode::Invalid
            | git2::ErrorCode::Uncommitted
            | git2::ErrorCode::Directory
            | git2::ErrorCode::MergeConflict
            | git2::ErrorCode::HashsumMismatch
            | git2::ErrorCode::IndexDirty
            | git2::ErrorCode::ApplyFail
            | git2::ErrorCode::Owner => GitStrategyError::RepositoryError {
                message: "There was a problem running an unspecified git2 command".into(),
                reason,
            },

            git2::ErrorCode::Auth | git2::ErrorCode::Certificate => {
                GitStrategyError::RemoteAuthenticationError {
                    message: "There was a problem running an unspecified git2 command".into(),
                    reason,
                }
            }

            _ => GitStrategyError::UnknownError {
                message: "There was a problem running an unspecified git2 command".into(),
                reason,
            },
        }
    }
}
