use std::{fs, process::Command};

use crate::dev::strategy::path::{PathStrategy, PathStrategyError};

use super::{GitStrategy, GitStrategyError, RepositoryChangeStatus, RepositoryLocation, Result};

// I may be using this later, but for now allow it to exist
#[allow(dead_code)]
#[derive(Debug)]
enum GitInternalChangeRepresentation<'a> {
    Untracked {
        path: &'a str,
    },
    Ignored {
        path: &'a str,
    },
    Ordinary {
        status: &'a str,
        sub_module: &'a str,
        file_mode_head: &'a str,
        file_mode_index: &'a str,
        file_mode_worktree: &'a str,
        object_name_head: &'a str,
        object_name_index: &'a str,
        path: &'a str,
    },
    Moved {
        status: &'a str,
        sub_module: &'a str,
        file_mode_head: &'a str,
        file_mode_index: &'a str,
        file_mode_worktree: &'a str,
        object_name_head: &'a str,
        object_name_index: &'a str,
        score: &'a str,
        path: &'a str,
    },
}

pub struct LocalGitStrategy<'a, T: PathStrategy> {
    path_strategy: &'a T,
}

impl<'a, T: PathStrategy> LocalGitStrategy<'a, T> {
    pub fn new(path_strategy: &'a T) -> Self {
        Self { path_strategy }
    }
}

impl<'a, T: PathStrategy> GitStrategy for LocalGitStrategy<'a, T> {
    fn clean<U>(&self, _repository: U) -> Result<()>
    where
        U: Into<RepositoryLocation>,
    {
        // TODO: This should be implemented later, I want to rethink cleaning first.
        // TODO: See <https://github.com/damymetzke/grass/issues/7>.
        Ok(())
    }

    fn clone<U, V>(&self, repository: U, remote: V) -> Result<()>
    where
        U: Into<RepositoryLocation>,
        V: AsRef<str>,
    {
        let repository: RepositoryLocation = repository.into();
        let repo_path = self.path_strategy.get_directory(repository.clone())?;

        fs::create_dir_all(&repo_path).map_err(|error| GitStrategyError::FileSystemError {
            message: String::from("Cannot create directory to clone into"),
            reason: error.to_string(),
            reasons: vec![],
        })?;

        let repo_path = repo_path.to_str().ok_or(GitStrategyError::UnknownError {
            message: String::from("Cannot resolve path"),
            reason: String::from("String conversion failed"),
        })?;

        let output = Command::new("git")
            .args(["-C", repo_path, "clone", remote.as_ref(), "."])
            .output()
            .map_err(|error| GitStrategyError::RemoteFetchError {
                message: String::from("Could not clone repository"),
                reason: error.to_string(),
            })?;

        if !matches!(output.status.code(), Some(0)) {
            return Err(GitStrategyError::RemoteFetchError {
                message: String::from("Error when running git clone"),
                reason: String::from_utf8(output.stderr)
                    .unwrap_or(String::from("stderr is not valid utf8")),
            });
        }

        Ok(())
    }

    fn get_changes<U>(&self, repository: U) -> Result<RepositoryChangeStatus>
    where
        U: Into<RepositoryLocation>,
    {
        let repository_location = repository.into();
        let repository_path = self
            .path_strategy
            .get_directory(repository_location.clone())?;

        let output = Command::new("git")
            .arg("-C")
            .arg(repository_path)
            .arg("status")
            .arg("--porcelain=v2")
            .output()
            .map_err(|error| GitStrategyError::RemoteFetchError {
                message: String::from("Could not clone repository"),
                reason: error.to_string(),
            })?;

        let output =
            String::from_utf8(output.stdout).map_err(|_| GitStrategyError::UnknownError {
                message: String::from("Cannot parse git output"),
                reason: String::from("Output is not valid utf8"),
            })?;

        let output: Box<_> = output.split('\n').filter_map(|line| {
            if line.chars().all(|char| char.is_whitespace()) {
                return None;
            };

            if let Some(path) = line.strip_prefix("? ") {
                return Some(GitInternalChangeRepresentation::Untracked { path });
            };

            if let Some(path) = line.strip_prefix("! ") {
                return Some(GitInternalChangeRepresentation::Ignored { path });
            };

            if let Some(content) = line.strip_prefix("1 ") {
                let parts: Box<_> = content.splitn(8, ' ').collect();
                if let[status, sub_module, file_mode_head, file_mode_index, file_mode_worktree, object_name_head, object_name_index, path] = parts.as_ref() {
                    return Some(GitInternalChangeRepresentation::Ordinary { status, sub_module, file_mode_head, file_mode_index, file_mode_worktree, object_name_head, object_name_index, path })
                }
            };

            if let Some(content) = line.strip_prefix("2 ") {
                let parts: Box<_> = content.splitn(9, ' ').collect();
                if let[status, sub_module, file_mode_head, file_mode_index, file_mode_worktree, object_name_head, object_name_index, score, path] = parts.as_ref() {
                    return Some(GitInternalChangeRepresentation::Moved { status, sub_module, file_mode_head, file_mode_index, file_mode_worktree, object_name_head, object_name_index, score, path })
                }
            };

            None
        }).filter(|change| !matches!(change, GitInternalChangeRepresentation::Ignored{..})).collect();

        if output.is_empty() {
            return Ok(RepositoryChangeStatus::UpToDate);
        }

        Ok(RepositoryChangeStatus::UncommittedChanges {
            num_changes: output.len(),
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
