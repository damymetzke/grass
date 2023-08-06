use crate::dev::public::api::RepositoryLocation;

use super::{GitStrategy, GitStrategyError, RepositoryChangeStatus, Result};

/// Strategy used for mocking
///
/// # Data
///
/// This strategy simulates the following data.
/// Each repository has an associated state.
/// The first level are the categories.
/// One indentation are the repositories.
///
/// - all_good (all of these are working and have no changes)
///   - first
///   - second
///   - third
/// - with_changes
///   - first (no changes)
///   - second (no repository)
///   - third (9 uncommitted changes)
/// - with_error
///   - first (invalid repository)
///   - second (unsufficient file permissions)
///
/// # remotes
///
/// This stratgy simulates the following remotes:
///
/// - good_remote (no errors)
/// - no_access (authentication error)
/// - bad_response (invalid response)
#[derive(Default)]
pub struct MockGitStrategy;

impl GitStrategy for MockGitStrategy {
    fn clean<T>(&self, repository: T) -> Result<()>
    where
        T: Into<RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_ref(), repository.repository.as_str());
        match repository {
            ("all_good" | "with_changes", _) => Ok(()),
            ("with_error", "first") => Err(GitStrategyError::RepositoryError {
                message: "Mocked error".into(),
                reason: "invalid repository".into(),
            }),
            ("with_error", "second") => Err(GitStrategyError::FileSystemError {
                message: "Mocked error".into(),
                reason: "insufficient permission".into(),
                reasons: vec![],
            }),
            _ => Err(GitStrategyError::RepositoryNotFound {
                message: "Mocked error".into(),
                reason: "cannot find repository".into(),
            }),
        }
    }

    fn clone<T, U>(&self, repository: T, remote: U) -> Result<()>
    where
        T: Into<RepositoryLocation>,
        U: AsRef<str>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_ref(), repository.repository.as_str());
        match repository {
            ("all_good" | "with_changes" | "with_error", "first" | "second")
            | ("all_good" | "with_changes", "third") => Err(GitStrategyError::RepositryExists {
                message: "Mocked error".into(),
                reason: "Can't fetch repository because it already exists locally".into(),
            }),
            ("all_good" | "with_changes" | "wth_error", _) => Ok(()),
            _ => Err(GitStrategyError::RepositoryNotFound {
                message: "Mocked error".into(),
                reason: "Category does not exist".into(),
            }),
        }?;

        match remote.as_ref() {
            "good_remote" => Ok(()),
            "no_access" => Err(GitStrategyError::RemoteAuthenticationError {
                message: "Mocked error".into(),
                reason: "You are not authorized to access this remote".into(),
            }),
            "bad_response" => Err(GitStrategyError::RemoteFetchError {
                message: "Mocked error".into(),
                reason: "The remote returned an invalid response".into(),
            }),
            _ => Err(GitStrategyError::RemoteFetchError {
                message: "Mocked error".into(),
                reason: "Could not access the remote".into(),
            }),
        }
    }

    fn get_changes<T>(&self, repository: T) -> Result<RepositoryChangeStatus>
    where
        T: Into<RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_ref(), repository.repository.as_str());
        match repository {
            ("all_good", "first" | "second" | "third") => Ok(RepositoryChangeStatus::UpToDate),
            ("with_changes", "first") => Ok(RepositoryChangeStatus::UpToDate),
            ("with_changes", "second") => Ok(RepositoryChangeStatus::NoRepository),
            ("with_changes", "third") => {
                Ok(RepositoryChangeStatus::UncommittedChanges { num_changes: 9 })
            }
            ("with_error", "first") => Err(GitStrategyError::RepositoryError {
                message: "Mocked error".into(),
                reason: "invalid repository".into(),
            }),
            ("with_error", "second") => Err(GitStrategyError::FileSystemError {
                message: "Mocked error".into(),
                reason: "insufficient permission".into(),
                reasons: vec![],
            }),
            ("all_good" | "with_changes" | "with_error", _) => {
                Err(GitStrategyError::RepositoryNotFound {
                    message: "Mocked error".into(),
                    reason: "repository not found".into(),
                })
            }
            _ => Err(GitStrategyError::RepositoryNotFound {
                message: "Mocked error".into(),
                reason: "category not found".into(),
            }),
        }
    }
}
